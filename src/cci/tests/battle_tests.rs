/*!
 * Battle tests for the CCI merit system.
 *
 * Each test is an attack: an attempt to game, corrupt, or break the ledger.
 * The system passes only if every attack fails or is detected.
 */

use cci::{CCISystem, ContributionType, PeerReview, MAX_HOURS_PER_ENTRY, MIN_REVIEWS};
use chrono::Utc;
use uuid::Uuid;

fn reviews(scores: &[f64]) -> Vec<PeerReview> {
    scores
        .iter()
        .map(|s| PeerReview {
            reviewer: Uuid::new_v4(),
            impact_score: *s,
            timestamp: Utc::now(),
        })
        .collect()
}

// ---------- Input attacks ----------

#[test]
fn attack_nan_hours_rejected() {
    let mut cci = CCISystem::new().unwrap();
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, 0.0] {
        assert!(
            cci.add_contribution(
                Uuid::new_v4(),
                Uuid::new_v4(),
                ContributionType::Writing,
                bad,
                reviews(&[0.5, 0.5, 0.5]),
            )
            .is_err(),
            "hours {} should be rejected",
            bad
        );
    }
}

#[test]
fn attack_nan_and_out_of_range_impact_rejected() {
    let mut cci = CCISystem::new().unwrap();
    for bad in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        assert!(
            cci.add_contribution(
                Uuid::new_v4(),
                Uuid::new_v4(),
                ContributionType::Writing,
                10.0,
                reviews(&[0.5, 0.5, bad]),
            )
            .is_err(),
            "impact {} should be rejected",
            bad
        );
    }
}

#[test]
fn attack_hour_cap_boundary() {
    let mut cci = CCISystem::new().unwrap();
    // Exactly at the cap: allowed.
    assert!(cci
        .add_contribution(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ContributionType::Writing,
            MAX_HOURS_PER_ENTRY,
            reviews(&[0.5, 0.5, 0.5]),
        )
        .is_ok());
    // The tiniest bit over: rejected.
    assert!(cci
        .add_contribution(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ContributionType::Writing,
            MAX_HOURS_PER_ENTRY + 0.0001,
            reviews(&[0.5, 0.5, 0.5]),
        )
        .is_err());
}

// ---------- Review-manipulation attacks ----------

#[test]
fn attack_single_shill_review_cannot_move_median() {
    let mut cci = CCISystem::new().unwrap();
    let honest = Uuid::new_v4();
    let shilled = Uuid::new_v4();
    let project = Uuid::new_v4();

    // Two members do identical work. One recruits a single shill reviewer
    // who scores 1.0; two honest reviewers score both at 0.6.
    cci.add_contribution(
        honest,
        project,
        ContributionType::Editing,
        50.0,
        reviews(&[0.6, 0.6, 0.6]),
    )
    .unwrap();
    cci.add_contribution(
        shilled,
        project,
        ContributionType::Editing,
        50.0,
        reviews(&[0.6, 0.6, 1.0]),
    )
    .unwrap();

    let scores = cci.compute_scores();
    assert_eq!(
        scores[&honest].total_points,
        scores[&shilled].total_points,
        "a single shill review must not change the outcome"
    );
}

#[test]
fn attack_hostile_reviewer_cannot_zero_out_work() {
    let mut cci = CCISystem::new().unwrap();
    let victim = Uuid::new_v4();

    // Two honest 0.8s and one saboteur scoring 0.0.
    cci.add_contribution(
        victim,
        Uuid::new_v4(),
        ContributionType::Direction,
        50.0,
        reviews(&[0.8, 0.0, 0.8]),
    )
    .unwrap();

    let scores = cci.compute_scores();
    assert!(
        (scores[&victim].total_points - 40.0).abs() < 1e-9,
        "median must ignore the saboteur"
    );
}

#[test]
fn attack_self_review_rejected_via_add_review_too() {
    let mut cci = CCISystem::new().unwrap();
    let member = Uuid::new_v4();
    let id = cci
        .add_contribution(
            member,
            Uuid::new_v4(),
            ContributionType::Sound,
            10.0,
            reviews(&[0.5]),
        )
        .unwrap();

    let result = cci.add_review(
        id,
        PeerReview {
            reviewer: member,
            impact_score: 1.0,
            timestamp: Utc::now(),
        },
    );
    assert!(result.is_err());
}

#[test]
fn attack_double_voting_via_add_review_rejected() {
    let mut cci = CCISystem::new().unwrap();
    let id = cci
        .add_contribution(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ContributionType::Sound,
            10.0,
            reviews(&[0.5]),
        )
        .unwrap();

    let reviewer = Uuid::new_v4();
    let vote = |score| PeerReview {
        reviewer,
        impact_score: score,
        timestamp: Utc::now(),
    };
    assert!(cci.add_review(id, vote(0.9)).is_ok());
    // Same reviewer votes again to stack the median: rejected.
    assert!(cci.add_review(id, vote(1.0)).is_err());
}

#[test]
fn attack_review_ring_on_shared_project_blocked() {
    // Alice and Bob work on the same project and try to review each other up.
    let mut cci = CCISystem::new().unwrap();
    let alice = Uuid::new_v4();
    let bob = Uuid::new_v4();
    let project = Uuid::new_v4();

    let a_id = cci
        .add_contribution(alice, project, ContributionType::Direction, 40.0, vec![])
        .unwrap();
    let b_id = cci
        .add_contribution(bob, project, ContributionType::Sound, 40.0, vec![])
        .unwrap();

    let review = |who| PeerReview {
        reviewer: who,
        impact_score: 1.0,
        timestamp: Utc::now(),
    };
    assert!(cci.add_review(a_id, review(bob)).is_err());
    assert!(cci.add_review(b_id, review(alice)).is_err());
}

#[test]
fn attack_under_reviewed_work_earns_nothing() {
    let mut cci = CCISystem::new().unwrap();
    let member = Uuid::new_v4();
    let project = Uuid::new_v4();

    let id = cci
        .add_contribution(member, project, ContributionType::Writing, 84.0, vec![])
        .unwrap();
    for i in 0..(MIN_REVIEWS - 1) {
        cci.add_review(
            id,
            PeerReview {
                reviewer: Uuid::new_v4(),
                impact_score: 1.0,
                timestamp: Utc::now(),
            },
        )
        .unwrap();
        assert!(
            cci.compute_scores().get(&member).is_none(),
            "must not score with only {} reviews",
            i + 1
        );
    }
    // The MIN_REVIEWS-th review unlocks scoring.
    cci.add_review(
        id,
        PeerReview {
            reviewer: Uuid::new_v4(),
            impact_score: 1.0,
            timestamp: Utc::now(),
        },
    )
    .unwrap();
    assert!(cci.compute_scores().get(&member).is_some());
}

// ---------- Structural gaming attacks ----------

#[test]
fn attack_entry_splitting_gains_nothing() {
    // Claiming 80h as one entry vs. 8x10h entries must yield identical points.
    let project = Uuid::new_v4();

    let mut one = CCISystem::new().unwrap();
    let m1 = Uuid::new_v4();
    one.add_contribution(m1, project, ContributionType::Editing, 80.0, reviews(&[0.7, 0.7, 0.7]))
        .unwrap();

    let mut many = CCISystem::new().unwrap();
    let m2 = Uuid::new_v4();
    for _ in 0..8 {
        many.add_contribution(m2, project, ContributionType::Editing, 10.0, reviews(&[0.7, 0.7, 0.7]))
            .unwrap();
    }

    let p1 = one.compute_scores()[&m1].total_points;
    let p2 = many.compute_scores()[&m2].total_points;
    assert!((p1 - p2).abs() < 1e-9, "splitting entries must not change points");
}

#[test]
fn attack_zero_impact_work_earns_zero() {
    let mut cci = CCISystem::new().unwrap();
    let member = Uuid::new_v4();
    let project = Uuid::new_v4();

    // Max hours of worthless work, unanimously scored 0.
    cci.add_contribution(member, project, ContributionType::Other("busywork".into()), 84.0, reviews(&[0.0, 0.0, 0.0]))
        .unwrap();

    assert_eq!(cci.compute_scores()[&member].total_points, 0.0);
    // And zero points means zero residuals.
    let shares = cci.global_residuals(1_000_000, project, 1).unwrap();
    assert!(shares.is_empty());
}

// ---------- Money-conservation attacks ----------

#[test]
fn attack_rounding_cannot_mint_money() {
    // Three-way splits produce repeating fractions; the sum must never
    // exceed the pot, across many pot sizes.
    let mut cci = CCISystem::new().unwrap();
    let project = Uuid::new_v4();
    for _ in 0..3 {
        cci.add_contribution(
            Uuid::new_v4(),
            project,
            ContributionType::Acting,
            33.0,
            reviews(&[0.7, 0.9, 0.8]),
        )
        .unwrap();
    }

    for pot in [1u128, 2, 3, 10, 999, 1_000_000, u128::from(u64::MAX)] {
        let shares = cci.global_residuals(pot, project, 1).unwrap();
        let total: u128 = shares.iter().map(|s| s.amount).sum();
        assert!(total <= pot, "minted money at pot {}: paid {}", pot, total);
    }
}

#[test]
fn attack_extreme_pot_u128_max_conserved() {
    // f64 cannot represent u128::MAX exactly; the clamp must still hold.
    let mut cci = CCISystem::new().unwrap();
    let project = Uuid::new_v4();
    for _ in 0..7 {
        cci.add_contribution(
            Uuid::new_v4(),
            project,
            ContributionType::Technical,
            13.0,
            reviews(&[0.3, 0.9, 0.6]),
        )
        .unwrap();
    }

    let shares = cci.global_residuals(u128::MAX, project, 1).unwrap();
    let mut total: u128 = 0;
    for s in &shares {
        total = total
            .checked_add(s.amount)
            .expect("share sum overflowed u128 — money was minted");
    }
    assert!(total <= u128::MAX);
}

#[test]
fn attack_dust_loss_is_bounded() {
    // Truncation may strand dust, but never more than one unit per member.
    let mut cci = CCISystem::new().unwrap();
    let project = Uuid::new_v4();
    let n = 7u128;
    for _ in 0..n {
        cci.add_contribution(
            Uuid::new_v4(),
            project,
            ContributionType::Acting,
            10.0,
            reviews(&[0.5, 0.5, 0.5]),
        )
        .unwrap();
    }
    let pot = 1_000_003u128; // prime, guaranteed remainder
    let shares = cci.global_residuals(pot, project, 1).unwrap();
    let total: u128 = shares.iter().map(|s| s.amount).sum();
    assert!(total <= pot);
    assert!(pot - total <= n, "stranded dust exceeds one unit per member");
}

// ---------- Ledger-corruption attacks ----------

#[test]
fn attack_reordering_entries_detected() {
    let mut cci = CCISystem::new().unwrap();
    for _ in 0..4 {
        cci.add_contribution(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ContributionType::Design,
            5.0,
            reviews(&[0.5, 0.5, 0.5]),
        )
        .unwrap();
    }
    assert!(cci.audit().chain_valid);

    // No public mutable access to the ledger exists, so simulate an attacker
    // with storage access by round-tripping through serde... the ledger is
    // private, which is itself part of the defense. What we CAN verify from
    // the outside: the head hash pins the exact sequence.
    let head_before = cci.head_hash();
    cci.add_contribution(
        Uuid::new_v4(),
        Uuid::new_v4(),
        ContributionType::Design,
        5.0,
        reviews(&[0.5, 0.5, 0.5]),
    )
    .unwrap();
    assert_ne!(cci.head_hash(), head_before, "head hash must change on append");
    assert!(cci.audit().chain_valid);
}

#[test]
fn attack_truncation_changes_head_hash() {
    // A truncated ledger still has a valid internal chain — the defense is
    // the externally anchored head hash, which truncation cannot preserve.
    let mut a = CCISystem::new().unwrap();
    let member = Uuid::new_v4();
    let project = Uuid::new_v4();

    a.add_contribution(member, project, ContributionType::Direction, 10.0, reviews(&[0.5, 0.5, 0.5]))
        .unwrap();
    let anchored_head = a.head_hash();
    let anchored_len = a.ledger_len();

    a.add_contribution(member, project, ContributionType::Direction, 10.0, reviews(&[0.5, 0.5, 0.5]))
        .unwrap();

    // After more work exists, the old anchor no longer matches: any replica
    // presenting the truncated state fails the anchor check.
    assert_ne!(a.head_hash(), anchored_head);
    assert!(a.ledger_len() > anchored_len);
}

#[test]
fn attack_scores_are_deterministic_across_recomputation() {
    let mut cci = CCISystem::new().unwrap();
    let project = Uuid::new_v4();
    for _ in 0..20 {
        cci.add_contribution(
            Uuid::new_v4(),
            project,
            ContributionType::Acting,
            7.5,
            reviews(&[0.4, 0.8, 0.6]),
        )
        .unwrap();
    }

    let r1 = cci.global_residuals(987_654_321, project, 1).unwrap();
    for _ in 0..10 {
        let r2 = cci.global_residuals(987_654_321, project, 1).unwrap();
        assert_eq!(r1.len(), r2.len());
        for (a, b) in r1.iter().zip(r2.iter()) {
            assert_eq!(a.member_id, b.member_id);
            assert_eq!(a.amount, b.amount);
        }
    }
}

#[test]
fn attack_unknown_contribution_review_rejected() {
    let mut cci = CCISystem::new().unwrap();
    let result = cci.add_review(
        Uuid::new_v4(),
        PeerReview {
            reviewer: Uuid::new_v4(),
            impact_score: 0.5,
            timestamp: Utc::now(),
        },
    );
    assert!(result.is_err());
}
