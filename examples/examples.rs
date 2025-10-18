//! Comprehensive examples and data structures for keypath operations

use key_paths_derive::Keypaths;
use crate::prelude::*;
use std::collections::HashMap;

/// Example data structures for demonstrating keypath operations
#[derive(Keypaths, Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub address: Address,
    pub skills: Vec<String>,
    pub employment: Option<Employment>,
}

#[derive(Keypaths, Debug, Clone, PartialEq)]
pub struct Address {
    pub city: String,
    pub country: String,
    pub coordinates: Coordinates,
}

#[derive(Keypaths, Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Keypaths, Debug, Clone, PartialEq)]
pub struct Employment {
    pub company: String,
    pub position: String,
    pub salary: u32,
    pub department: Department,
}

#[derive(Keypaths, Debug, Clone, PartialEq)]
pub struct Department {
    pub name: String,
    pub budget: u32,
    pub manager: Option<String>,
}

/// Sample data for examples
pub fn sample_people() -> Vec<Person> {
    vec![
        Person {
            name: "Alice Johnson".to_string(),
            age: 30,
            address: Address {
                city: "New York".to_string(),
                country: "USA".to_string(),
                coordinates: Coordinates { lat: 40.7128, lng: -74.0060 },
            },
            skills: vec!["Rust".to_string(), "Python".to_string(), "JavaScript".to_string()],
            employment: Some(Employment {
                company: "TechCorp".to_string(),
                position: "Senior Engineer".to_string(),
                salary: 120000,
                department: Department {
                    name: "Engineering".to_string(),
                    budget: 1000000,
                    manager: Some("Bob Smith".to_string()),
                },
            }),
        },
        Person {
            name: "Bob Smith".to_string(),
            age: 35,
            address: Address {
                city: "San Francisco".to_string(),
                country: "USA".to_string(),
                coordinates: Coordinates { lat: 37.7749, lng: -122.4194 },
            },
            skills: vec!["Rust".to_string(), "Go".to_string(), "Kubernetes".to_string()],
            employment: Some(Employment {
                company: "TechCorp".to_string(),
                position: "Engineering Manager".to_string(),
                salary: 150000,
                department: Department {
                    name: "Engineering".to_string(),
                    budget: 1000000,
                    manager: None,
                },
            }),
        },
        Person {
            name: "Charlie Brown".to_string(),
            age: 28,
            address: Address {
                city: "London".to_string(),
                country: "UK".to_string(),
                coordinates: Coordinates { lat: 51.5074, lng: -0.1278 },
            },
            skills: vec!["Python".to_string(), "Machine Learning".to_string()],
            employment: Some(Employment {
                company: "DataCorp".to_string(),
                position: "Data Scientist".to_string(),
                salary: 80000,
                department: Department {
                    name: "Data Science".to_string(),
                    budget: 500000,
                    manager: Some("Diana Prince".to_string()),
                },
            }),
        },
        Person {
            name: "Diana Prince".to_string(),
            age: 32,
            address: Address {
                city: "Berlin".to_string(),
                country: "Germany".to_string(),
                coordinates: Coordinates { lat: 52.5200, lng: 13.4050 },
            },
            skills: vec!["Python".to_string(), "R".to_string(), "Statistics".to_string()],
            employment: Some(Employment {
                company: "DataCorp".to_string(),
                position: "Data Science Manager".to_string(),
                salary: 95000,
                department: Department {
                    name: "Data Science".to_string(),
                    budget: 500000,
                    manager: None,
                },
            }),
        },
        Person {
            name: "Eve Wilson".to_string(),
            age: 25,
            address: Address {
                city: "Tokyo".to_string(),
                country: "Japan".to_string(),
                coordinates: Coordinates { lat: 35.6762, lng: 139.6503 },
            },
            skills: vec!["JavaScript".to_string(), "React".to_string(), "Node.js".to_string()],
            employment: None,
        },
    ]
}

/// Basic keypath operations examples
pub mod basic_examples {
    use super::*;
    
    /// Example: Filter people by age
    pub fn filter_by_age_example() -> KeyPathResult<Vec<Person>> {
        let people = sample_people();
        let young_people: Vec<Person> = people
            .into_iter()
            .filter_by_keypath(Person::age_r(), |&age| age < 30)
            .collect();
        Ok(young_people)
    }
    
    /// Example: Map over names
    pub fn map_names_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let names: Vec<String> = people
            .into_iter()
            .map_keypath(Person::name_r(), |name| name.to_uppercase())
            .collect();
        Ok(names)
    }
    
    /// Example: Find person by name
    pub fn find_by_name_example() -> KeyPathResult<Option<Person>> {
        let people = sample_people();
        let found = people
            .into_iter()
            .find_by_keypath(Person::name_r(), |name| name.contains("Alice"))
            .unwrap();
        Ok(found)
    }
    
    /// Example: Group by country
    pub fn group_by_country_example() -> KeyPathResult<HashMap<String, Vec<Person>>> {
        let people = sample_people();
        let grouped: HashMap<String, Vec<Person>> = people
            .group_by_keypath(Person::address_r().then(Address::country_r()), |country| country.clone())
            .unwrap();
        Ok(grouped)
    }
    
    /// Example: Sort by age
    pub fn sort_by_age_example() -> KeyPathResult<Vec<Person>> {
        let mut people = sample_people();
        people.sort_by_keypath(Person::age_r(), |a, b| a.cmp(b)).unwrap();
        Ok(people)
    }
}

/// Advanced keypath operations examples
pub mod advanced_examples {
    use super::*;
    
    /// Example: Complex filtering and mapping
    pub fn complex_filtering_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let result: Vec<String> = people
            .into_iter()
            .filter_by_keypath(Person::age_r(), |&age| age >= 30)
            .filter_by_keypath(Person::employment_r(), |emp| emp.is_some())
            .map_keypath(Person::employment_r().then(Employment::company_r()), |company| {
                company.as_ref().unwrap().clone()
            })
            .collect();
        Ok(result)
    }
    
    /// Example: Nested keypath operations
    pub fn nested_keypath_example() -> KeyPathResult<Vec<f64>> {
        let people = sample_people();
        let latitudes: Vec<f64> = people
            .into_iter()
            .map_keypath(
                Person::address_r()
                    .then(Address::coordinates_r())
                    .then(Coordinates::lat_r()),
                |&lat| lat,
            )
            .collect();
        Ok(latitudes)
    }
    
    /// Example: Window operations
    pub fn window_operations_example() -> KeyPathResult<Vec<f64>> {
        let people = sample_people();
        let ages: Vec<u32> = people
            .collect_keypath(Person::age_r())
            .unwrap();
        
        let moving_averages: Vec<f64> = ages
            .window_by_keypath(|ages: &[u32]| {
                ages.iter().sum::<u32>() as f64 / ages.len() as f64
            }, 3)
            .unwrap();
        Ok(moving_averages)
    }
    
    /// Example: Rolling operations
    pub fn rolling_operations_example() -> KeyPathResult<Vec<u32>> {
        let people = sample_people();
        let ages: Vec<u32> = people
            .collect_keypath(Person::age_r())
            .unwrap();
        
        let rolling_sums: Vec<u32> = ages
            .rolling_by_keypath(|ages: &[u32]| {
                ages.iter().sum()
            }, 2)
            .unwrap();
        Ok(rolling_sums)
    }
}

/// Composable operations examples
pub mod composable_examples {
    use super::*;
    
    /// Example: Using pipe for function composition
    pub fn pipe_composition_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let result: Vec<String> = people
            .into_iter()
            .pipe(|iter| iter.filter_by_keypath(Person::age_r(), |&age| age < 35))
            .pipe(|iter| iter.map_keypath(Person::name_r(), |name| name.to_lowercase()))
            .collect();
        Ok(result)
    }
    
    /// Example: Using chain for complex operations
    pub fn chain_operations_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let result: Vec<String> = people
            .into_iter()
            .chain_keypath_ops()
            .filter_by_keypath(Person::employment_r(), |emp| emp.is_some())
            .map_keypath(Person::employment_r().then(Employment::department_r()).then(Department::name_r()), |dept| {
                dept.as_ref().unwrap().clone()
            })
            .collect();
        Ok(result)
    }
    
    /// Example: Conditional operations
    pub fn conditional_operations_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let result: Vec<String> = people
            .into_iter()
            .when_keypath(Person::age_r(), |&age| age >= 30, |iter| {
                iter.map_keypath(Person::name_r(), |name| format!("Senior: {}", name))
            })
            .unwrap();
        Ok(result)
    }
}

/// Performance examples
pub mod performance_examples {
    use super::*;
    
    /// Example: Efficient filtering with early termination
    pub fn efficient_filtering_example() -> KeyPathResult<Option<Person>> {
        let people = sample_people();
        let found = people
            .into_iter()
            .find_by_keypath(Person::skills_r(), |skills| {
                skills.contains(&"Rust".to_string())
            })
            .unwrap();
        Ok(found)
    }
    
    /// Example: Batch operations
    pub fn batch_operations_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let all_skills: Vec<String> = people
            .into_iter()
            .flat_map(|person| {
                person.skills.into_iter()
            })
            .collect();
        
        let unique_skills: std::collections::HashSet<String> = all_skills.into_iter().collect();
        Ok(unique_skills.into_iter().collect())
    }
}

/// Error handling examples
pub mod error_handling_examples {
    use super::*;
    
    /// Example: Safe keypath access with error handling
    pub fn safe_keypath_access_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let mut result = Vec::new();
        
        for person in people {
            match person.get_at_keypath(Person::employment_r().then(Employment::company_r())) {
                Ok(company) => {
                    if let Some(company_name) = company {
                        result.push(company_name.clone());
                    }
                }
                Err(e) => {
                    eprintln!("Error accessing company: {}", e);
                }
            }
        }
        
        Ok(result)
    }
    
    /// Example: Graceful degradation
    pub fn graceful_degradation_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let result: Vec<String> = people
            .into_iter()
            .filter_map(|person| {
                person.get_at_keypath(Person::employment_r().then(Employment::company_r()))
                    .ok()
                    .and_then(|company| company.as_ref().cloned())
            })
            .collect();
        Ok(result)
    }
}

/// Integration examples
pub mod integration_examples {
    use super::*;
    
    /// Example: Working with serde (if feature enabled)
    #[cfg(feature = "serde")]
    pub fn serde_integration_example() -> KeyPathResult<String> {
        let people = sample_people();
        let json = serde_json::to_string(&people)
            .map_err(|e| KeyPathError::RuntimeFailure {
                message: format!("Serialization failed: {}", e),
            })?;
        Ok(json)
    }
    
    /// Example: Working with async operations (if feature enabled)
    #[cfg(feature = "async")]
    pub async fn async_integration_example() -> KeyPathResult<Vec<String>> {
        let people = sample_people();
        let names: Vec<String> = people
            .into_iter()
            .map_keypath(Person::name_r(), |name| name.clone())
            .collect();
        
        // Simulate async operation
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Ok(names)
    }
}

/// Utility functions for examples
pub mod utils {
    use super::*;
    
    /// Create a sample person for testing
    pub fn create_sample_person() -> Person {
        Person {
            name: "Test Person".to_string(),
            age: 25,
            address: Address {
                city: "Test City".to_string(),
                country: "Test Country".to_string(),
                coordinates: Coordinates { lat: 0.0, lng: 0.0 },
            },
            skills: vec!["Rust".to_string()],
            employment: None,
        }
    }
    
    /// Create a large dataset for performance testing
    pub fn create_large_dataset(size: usize) -> Vec<Person> {
        (0..size)
            .map(|i| Person {
                name: format!("Person {}", i),
                age: 20 + (i % 50),
                address: Address {
                    city: format!("City {}", i % 10),
                    country: if i % 2 == 0 { "USA".to_string() } else { "UK".to_string() },
                    coordinates: Coordinates { lat: 0.0, lng: 0.0 },
                },
                skills: vec!["Rust".to_string(), "Python".to_string()],
                employment: Some(Employment {
                    company: format!("Company {}", i % 5),
                    position: "Engineer".to_string(),
                    salary: 50000 + (i * 1000),
                    department: Department {
                        name: "Engineering".to_string(),
                        budget: 1000000,
                        manager: None,
                    },
                }),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_filtering() {
        let result = basic_examples::filter_by_age_example().unwrap();
        assert_eq!(result.len(), 2); // Eve (25) and Charlie (28)
    }
    
    #[test]
    fn test_name_mapping() {
        let result = basic_examples::map_names_example().unwrap();
        assert!(result.contains(&"ALICE JOHNSON".to_string()));
    }
    
    #[test]
    fn test_finding_by_name() {
        let result = basic_examples::find_by_name_example().unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Alice Johnson");
    }
    
    #[test]
    fn test_grouping_by_country() {
        let result = basic_examples::group_by_country_example().unwrap();
        assert!(result.contains_key("USA"));
        assert!(result.contains_key("UK"));
        assert!(result.contains_key("Germany"));
        assert!(result.contains_key("Japan"));
    }
    
    #[test]
    fn test_sorting_by_age() {
        let result = basic_examples::sort_by_age_example().unwrap();
        assert_eq!(result[0].age, 25); // Eve
        assert_eq!(result[4].age, 35); // Bob
    }
}
