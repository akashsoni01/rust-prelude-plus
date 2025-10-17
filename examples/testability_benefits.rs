use rust_prelude_plus::prelude::*;
use key_paths_derive::Keypath;
use std::collections::HashMap;

#[derive(Keypath, Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
    is_active: bool,
    subscription_tier: String,
    last_login: String,
    preferences: UserPreferences,
}

#[derive(Keypath, Debug, Clone, PartialEq)]
struct UserPreferences {
    theme: String,
    notifications: bool,
    language: String,
}

// Business logic functions that use KeyPaths
fn get_active_users(users: &[User]) -> KeyPathResult<Vec<User>> {
    filter_by_keypath(users.to_vec(), User::is_active(), |&active| active)
}

fn get_users_by_tier(users: &[User], tier: &str) -> KeyPathResult<Vec<User>> {
    filter_by_keypath(users.to_vec(), User::subscription_tier(), |user_tier| user_tier == tier)
}

fn calculate_average_age(users: &[User]) -> KeyPathResult<f64> {
    let total_age = fold_keypath(users.to_vec(), User::age(), 0, |acc, &age| acc + age)?;
    Ok(total_age as f64 / users.len() as f64)
}

fn get_user_emails(users: &[User]) -> KeyPathResult<Vec<String>> {
    collect_keypath(users.to_vec(), User::email())
}

fn group_users_by_tier(users: &[User]) -> KeyPathResult<HashMap<String, Vec<User>>> {
    group_by_keypath(users, User::subscription_tier(), |tier| tier.clone())
}

fn get_users_with_notifications_enabled(users: &[User]) -> KeyPathResult<Vec<User>> {
    filter_by_keypath(users.to_vec(), User::preferences().then(UserPreferences::notifications()), |&notifications| notifications)
}

fn main() {
    println!("=== KeyPaths and Testability Benefits ===\n");

    // Sample test data
    let users = vec![
        User {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 30,
            is_active: true,
            subscription_tier: "premium".to_string(),
            last_login: "2024-01-15".to_string(),
            preferences: UserPreferences {
                theme: "dark".to_string(),
                notifications: true,
                language: "en".to_string(),
            },
        },
        User {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            age: 25,
            is_active: true,
            subscription_tier: "basic".to_string(),
            last_login: "2024-01-14".to_string(),
            preferences: UserPreferences {
                theme: "light".to_string(),
                notifications: false,
                language: "en".to_string(),
            },
        },
        User {
            id: 3,
            name: "Charlie Brown".to_string(),
            email: "charlie@example.com".to_string(),
            age: 35,
            is_active: false,
            subscription_tier: "premium".to_string(),
            last_login: "2024-01-10".to_string(),
            preferences: UserPreferences {
                theme: "dark".to_string(),
                notifications: true,
                language: "es".to_string(),
            },
        },
    ];

    // Demonstrate business logic functions
    println!("1. BUSINESS LOGIC FUNCTIONS:");
    
    let active_users = get_active_users(&users).unwrap();
    println!("   Active users: {}", active_users.len());
    
    let premium_users = get_users_by_tier(&users, "premium").unwrap();
    println!("   Premium users: {}", premium_users.len());
    
    let avg_age = calculate_average_age(&users).unwrap();
    println!("   Average age: {:.1}", avg_age);
    
    let emails = get_user_emails(&users).unwrap();
    println!("   User emails: {:?}", emails);
    
    let tier_groups = group_users_by_tier(&users).unwrap();
    println!("   Users by tier:");
    for (tier, tier_users) in &tier_groups {
        println!("     {}: {} users", tier, tier_users.len());
    }
    
    let notification_users = get_users_with_notifications_enabled(&users).unwrap();
    println!("   Users with notifications enabled: {}", notification_users.len());
    
    println!("\n2. TESTABILITY BENEFITS:");
    println!("   ✅ Pure Functions: All business logic functions are pure");
    println!("   ✅ Predictable: Same input always produces same output");
    println!("   ✅ Isolated: Functions don't depend on external state");
    println!("   ✅ Composable: Functions can be easily combined and tested");
    println!("   ✅ Type-safe: Compile-time guarantees prevent runtime errors");
    println!("   ✅ Mockable: Easy to create test data with KeyPaths");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(id: u32, name: &str, email: &str, age: u32, is_active: bool, tier: &str) -> User {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            age,
            is_active,
            subscription_tier: tier.to_string(),
            last_login: "2024-01-15".to_string(),
            preferences: UserPreferences {
                theme: "dark".to_string(),
                notifications: true,
                language: "en".to_string(),
            },
        }
    }

    #[test]
    fn test_get_active_users() {
        // Arrange
        let users = vec![
            create_test_user(1, "Alice", "alice@test.com", 30, true, "premium"),
            create_test_user(2, "Bob", "bob@test.com", 25, false, "basic"),
            create_test_user(3, "Charlie", "charlie@test.com", 35, true, "premium"),
        ];

        // Act
        let result = get_active_users(&users).unwrap();

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Alice");
        assert_eq!(result[1].name, "Charlie");
    }

    #[test]
    fn test_get_users_by_tier() {
        // Arrange
        let users = vec![
            create_test_user(1, "Alice", "alice@test.com", 30, true, "premium"),
            create_test_user(2, "Bob", "bob@test.com", 25, true, "basic"),
            create_test_user(3, "Charlie", "charlie@test.com", 35, true, "premium"),
        ];

        // Act
        let premium_users = get_users_by_tier(&users, "premium").unwrap();
        let basic_users = get_users_by_tier(&users, "basic").unwrap();

        // Assert
        assert_eq!(premium_users.len(), 2);
        assert_eq!(basic_users.len(), 1);
        assert_eq!(premium_users[0].name, "Alice");
        assert_eq!(premium_users[1].name, "Charlie");
        assert_eq!(basic_users[0].name, "Bob");
    }

    #[test]
    fn test_calculate_average_age() {
        // Arrange
        let users = vec![
            create_test_user(1, "Alice", "alice@test.com", 30, true, "premium"),
            create_test_user(2, "Bob", "bob@test.com", 25, true, "basic"),
            create_test_user(3, "Charlie", "charlie@test.com", 35, true, "premium"),
        ];

        // Act
        let result = calculate_average_age(&users).unwrap();

        // Assert
        assert_eq!(result, 30.0); // (30 + 25 + 35) / 3 = 30
    }

    #[test]
    fn test_get_user_emails() {
        // Arrange
        let users = vec![
            create_test_user(1, "Alice", "alice@test.com", 30, true, "premium"),
            create_test_user(2, "Bob", "bob@test.com", 25, true, "basic"),
        ];

        // Act
        let result = get_user_emails(&users).unwrap();

        // Assert
        assert_eq!(result, vec!["alice@test.com", "bob@test.com"]);
    }

    #[test]
    fn test_group_users_by_tier() {
        // Arrange
        let users = vec![
            create_test_user(1, "Alice", "alice@test.com", 30, true, "premium"),
            create_test_user(2, "Bob", "bob@test.com", 25, true, "basic"),
            create_test_user(3, "Charlie", "charlie@test.com", 35, true, "premium"),
        ];

        // Act
        let result = group_users_by_tier(&users).unwrap();

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result["premium"].len(), 2);
        assert_eq!(result["basic"].len(), 1);
        assert_eq!(result["premium"][0].name, "Alice");
        assert_eq!(result["premium"][1].name, "Charlie");
        assert_eq!(result["basic"][0].name, "Bob");
    }

    #[test]
    fn test_get_users_with_notifications_enabled() {
        // Arrange
        let mut user1 = create_test_user(1, "Alice", "alice@test.com", 30, true, "premium");
        user1.preferences.notifications = true;
        
        let mut user2 = create_test_user(2, "Bob", "bob@test.com", 25, true, "basic");
        user2.preferences.notifications = false;
        
        let mut user3 = create_test_user(3, "Charlie", "charlie@test.com", 35, true, "premium");
        user3.preferences.notifications = true;
        
        let users = vec![user1, user2, user3];

        // Act
        let result = get_users_with_notifications_enabled(&users).unwrap();

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Alice");
        assert_eq!(result[1].name, "Charlie");
    }

    #[test]
    fn test_keypath_type_safety() {
        // This test demonstrates compile-time type safety
        let users = vec![create_test_user(1, "Alice", "alice@test.com", 30, true, "premium")];
        
        // These operations are guaranteed to work at compile time
        let names = collect_keypath(users.clone(), User::name()).unwrap();
        let ages = collect_keypath(users.clone(), User::age()).unwrap();
        let emails = collect_keypath(users.clone(), User::email()).unwrap();
        
        assert_eq!(names, vec!["Alice"]);
        assert_eq!(ages, vec![30]);
        assert_eq!(emails, vec!["alice@test.com"]);
    }

    #[test]
    fn test_keypath_error_handling() {
        // This test demonstrates built-in error handling
        let users = vec![create_test_user(1, "Alice", "alice@test.com", 30, true, "premium")];
        
        // KeyPath operations return Result types for proper error handling
        let result: KeyPathResult<Vec<String>> = collect_keypath(users, User::name());
        assert!(result.is_ok());
        
        let names = result.unwrap();
        assert_eq!(names, vec!["Alice"]);
    }

    #[test]
    fn test_keypath_composability() {
        // This test demonstrates how KeyPaths enable composable testing
        let users = vec![
            create_test_user(1, "Alice", "alice@test.com", 30, true, "premium"),
            create_test_user(2, "Bob", "bob@test.com", 25, false, "basic"),
            create_test_user(3, "Charlie", "charlie@test.com", 35, true, "premium"),
        ];

        // Compose multiple operations
        let active_users = get_active_users(&users).unwrap();
        let premium_active_users = get_users_by_tier(&active_users, "premium").unwrap();
        let premium_active_emails = get_user_emails(&premium_active_users).unwrap();

        // Assert
        assert_eq!(premium_active_emails, vec!["alice@test.com"]);
    }

    #[test]
    fn test_keypath_mock_data_generation() {
        // This test demonstrates easy mock data generation with KeyPaths
        let mock_users = (1..=5)
            .map(|i| create_test_user(
                i,
                &format!("User{}", i),
                &format!("user{}@test.com", i),
                20 + i * 5,
                i % 2 == 0,
                if i % 3 == 0 { "premium" } else { "basic" }
            ))
            .collect::<Vec<_>>();

        // Test with generated data
        let active_count = get_active_users(&mock_users).unwrap().len();
        let premium_count = get_users_by_tier(&mock_users, "premium").unwrap().len();
        let avg_age = calculate_average_age(&mock_users).unwrap();

        // Assert
        assert_eq!(active_count, 2); // Users 2 and 4
        assert_eq!(premium_count, 1); // User 3
        assert_eq!(avg_age, 30.0); // (25 + 30 + 35 + 40 + 45) / 5
    }
}
