use crate::types::{EnemyType, Position};

#[test]
fn test_position_distance() {
    let p1 = Position::new(0.0, 0.0);
    let p2 = Position::new(3.0, 4.0);

    assert!((p1.distance_to(&p2) - 5.0).abs() < 0.001);
}

#[test]
fn test_position_ring_calculation() {
    let ring_radius = 200.0;

    // Center should be ring 1 (min value)
    let center = Position::new(0.0, 0.0);
    assert_eq!(center.ring(ring_radius), 1);

    // Within first ring boundary (0-199)
    let p1 = Position::new(150.0, 0.0);
    assert_eq!(p1.ring(ring_radius), 1);

    // At ring 1 boundary (200-399)
    let p1_edge = Position::new(200.0, 0.0);
    assert_eq!(p1_edge.ring(ring_radius), 1);

    // Ring 2 (400-599)
    let p2 = Position::new(450.0, 0.0);
    assert_eq!(p2.ring(ring_radius), 2);

    // Ring 5 (1000+)
    let p5 = Position::new(1050.0, 0.0);
    assert_eq!(p5.ring(ring_radius), 5);
}

#[test]
fn test_position_move_towards() {
    let mut pos = Position::new(0.0, 0.0);
    let target = Position::new(10.0, 0.0);
    let speed = 5.0;
    let delta = 1.0;

    pos.move_towards(&target, speed, delta);

    // Should move 5 units towards target
    assert!((pos.x - 5.0).abs() < 0.001);
    assert!(pos.y.abs() < 0.001);
}

#[test]
fn test_enemy_stats_scaling() {
    let goblin = EnemyType::Goblin;

    let ring1_stats = goblin.stats_for_ring(1);
    let ring5_stats = goblin.stats_for_ring(5);

    // Ring 5 should have higher stats than ring 1
    assert!(ring5_stats.max_health > ring1_stats.max_health);
    assert!(ring5_stats.damage > ring1_stats.damage);
    assert!(ring5_stats.movement_speed > ring1_stats.movement_speed);
}

#[test]
fn test_all_enemy_types_exist() {
    let types = EnemyType::all();
    assert_eq!(types.len(), 10);

    // Verify all types can generate stats
    for enemy_type in types {
        let stats = enemy_type.stats_for_ring(1);
        assert!(stats.max_health > 0.0);
        assert!(stats.damage > 0.0);
        assert!(stats.movement_speed > 0.0);
    }
}
