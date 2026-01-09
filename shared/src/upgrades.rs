use serde::{Deserialize, Serialize};

/// Upgrade types available in the game (inspired by Vampire Survivors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeType {
    // Weapon upgrades
    IncreaseDamage,
    IncreaseAttackSpeed,
    IncreaseProjectileSpeed,
    MultiShot,          // Fire multiple projectiles
    PiercingShots,      // Projectiles pierce through enemies
    
    // Stat upgrades
    IncreaseMaxHealth,
    IncreaseMovementSpeed,
    HealthRegeneration,
    
    // Passive abilities
    PickupRadius,       // Increase XP pickup radius
    Magnet,             // Auto-collect XP
    Armor,              // Reduce damage taken
    Luck,               // Better drops/bonuses
}

impl UpgradeType {
    pub fn name(&self) -> &str {
        match self {
            UpgradeType::IncreaseDamage => "Damage+",
            UpgradeType::IncreaseAttackSpeed => "Attack Speed+",
            UpgradeType::IncreaseProjectileSpeed => "Projectile Speed+",
            UpgradeType::MultiShot => "Multi Shot",
            UpgradeType::PiercingShots => "Piercing Shots",
            UpgradeType::IncreaseMaxHealth => "Max Health+",
            UpgradeType::IncreaseMovementSpeed => "Move Speed+",
            UpgradeType::HealthRegeneration => "HP Regeneration",
            UpgradeType::PickupRadius => "Pickup Radius+",
            UpgradeType::Magnet => "Magnet",
            UpgradeType::Armor => "Armor",
            UpgradeType::Luck => "Luck",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            UpgradeType::IncreaseDamage => "Increase damage by 20%",
            UpgradeType::IncreaseAttackSpeed => "Increase attack speed by 15%",
            UpgradeType::IncreaseProjectileSpeed => "Increase projectile speed by 25%",
            UpgradeType::MultiShot => "Fire 2 additional projectiles",
            UpgradeType::PiercingShots => "Projectiles pierce through 1 enemy",
            UpgradeType::IncreaseMaxHealth => "Increase max health by 25%",
            UpgradeType::IncreaseMovementSpeed => "Increase movement speed by 10%",
            UpgradeType::HealthRegeneration => "Regenerate 1 HP per second",
            UpgradeType::PickupRadius => "Increase pickup radius by 50%",
            UpgradeType::Magnet => "Automatically collect nearby XP",
            UpgradeType::Armor => "Reduce damage taken by 10%",
            UpgradeType::Luck => "Increase luck by 10%",
        }
    }

    /// Get a random selection of upgrades (3 choices)
    pub fn random_choices(exclude: &[UpgradeType]) -> Vec<UpgradeType> {
        use rand::seq::SliceRandom;
        let all: Vec<UpgradeType> = vec![
            UpgradeType::IncreaseDamage,
            UpgradeType::IncreaseAttackSpeed,
            UpgradeType::IncreaseProjectileSpeed,
            UpgradeType::MultiShot,
            UpgradeType::PiercingShots,
            UpgradeType::IncreaseMaxHealth,
            UpgradeType::IncreaseMovementSpeed,
            UpgradeType::HealthRegeneration,
            UpgradeType::PickupRadius,
            UpgradeType::Magnet,
            UpgradeType::Armor,
            UpgradeType::Luck,
        ];
        
        let mut available: Vec<UpgradeType> = all
            .into_iter()
            .filter(|u| !exclude.contains(u))
            .collect();
        
        let mut rng = rand::thread_rng();
        available.shuffle(&mut rng);
        available.into_iter().take(3).collect()
    }
}

/// Player upgrade state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerUpgrades {
    pub damage_level: u32,
    pub attack_speed_level: u32,
    pub projectile_speed_level: u32,
    pub multi_shot_level: u32,
    pub piercing_level: u32,
    pub max_health_level: u32,
    pub movement_speed_level: u32,
    pub regen_level: u32,
    pub pickup_radius_level: u32,
    pub has_magnet: bool,
    pub armor_level: u32,
    pub luck_level: u32,
}

impl Default for PlayerUpgrades {
    fn default() -> Self {
        Self {
            damage_level: 0,
            attack_speed_level: 0,
            projectile_speed_level: 0,
            multi_shot_level: 0,
            piercing_level: 0,
            max_health_level: 0,
            movement_speed_level: 0,
            regen_level: 0,
            pickup_radius_level: 0,
            has_magnet: false,
            armor_level: 0,
            luck_level: 0,
        }
    }
}

impl PlayerUpgrades {
    pub fn apply_upgrade(&mut self, upgrade: UpgradeType) {
        match upgrade {
            UpgradeType::IncreaseDamage => self.damage_level += 1,
            UpgradeType::IncreaseAttackSpeed => self.attack_speed_level += 1,
            UpgradeType::IncreaseProjectileSpeed => self.projectile_speed_level += 1,
            UpgradeType::MultiShot => self.multi_shot_level += 1,
            UpgradeType::PiercingShots => self.piercing_level += 1,
            UpgradeType::IncreaseMaxHealth => self.max_health_level += 1,
            UpgradeType::IncreaseMovementSpeed => self.movement_speed_level += 1,
            UpgradeType::HealthRegeneration => self.regen_level += 1,
            UpgradeType::PickupRadius => self.pickup_radius_level += 1,
            UpgradeType::Magnet => self.has_magnet = true,
            UpgradeType::Armor => self.armor_level += 1,
            UpgradeType::Luck => self.luck_level += 1,
        }
    }

    /// Calculate effective damage multiplier
    pub fn damage_multiplier(&self) -> f32 {
        1.0 + (self.damage_level as f32 * 0.2)
    }

    /// Calculate effective attack speed multiplier
    pub fn attack_speed_multiplier(&self) -> f32 {
        1.0 + (self.attack_speed_level as f32 * 0.15)
    }

    /// Calculate effective movement speed multiplier
    pub fn movement_speed_multiplier(&self) -> f32 {
        1.0 + (self.movement_speed_level as f32 * 0.1)
    }

    /// Calculate effective projectile speed multiplier
    pub fn projectile_speed_multiplier(&self) -> f32 {
        1.0 + (self.projectile_speed_level as f32 * 0.25)
    }

    /// Calculate damage reduction from armor
    pub fn damage_reduction(&self) -> f32 {
        (self.armor_level as f32 * 0.1).min(0.75) // Max 75% reduction
    }

    /// Calculate number of additional projectiles
    pub fn extra_projectiles(&self) -> u32 {
        self.multi_shot_level * 2
    }
}
