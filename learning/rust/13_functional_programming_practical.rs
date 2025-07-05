// Rust 関数型プログラミング - 実践編
// 実際のプロジェクトでの関数型プログラミング活用

use std::collections::HashMap;

// 1. ログ解析システム
#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
    source: String,
}

impl LogEntry {
    fn parse(line: &str) -> Option<LogEntry> {
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() >= 4 {
            Some(LogEntry {
                timestamp: parts[0].to_string(),
                level: parts[1].to_string(),
                message: parts[2].to_string(),
                source: parts[3].to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LogAnalytics {
    total_entries: usize,
    error_count: usize,
    warning_count: usize,
    source_distribution: HashMap<String, usize>,
    hourly_distribution: HashMap<String, usize>,
}

fn analyze_logs(log_entries: Vec<LogEntry>) -> LogAnalytics {
    let total_entries = log_entries.len();
    
    // 関数型的なデータ処理
    let error_count = log_entries.iter()
        .filter(|entry| entry.level == "ERROR")
        .count();
    
    let warning_count = log_entries.iter()
        .filter(|entry| entry.level == "WARN")
        .count();
    
    let source_distribution = log_entries.iter()
        .fold(HashMap::new(), |mut acc, entry| {
            *acc.entry(entry.source.clone()).or_insert(0) += 1;
            acc
        });
    
    let hourly_distribution = log_entries.iter()
        .filter_map(|entry| {
            entry.timestamp.split('T').nth(1)
                .and_then(|time| time.split(':').next())
                .map(|hour| hour.to_string())
        })
        .fold(HashMap::new(), |mut acc, hour| {
            *acc.entry(hour).or_insert(0) += 1;
            acc
        });
    
    LogAnalytics {
        total_entries,
        error_count,
        warning_count,
        source_distribution,
        hourly_distribution,
    }
}

#[allow(unused)]
fn log_analysis_example() {
    let sample_logs = vec![
        "2024-01-01T10:30:00 | INFO | Application started | main.rs",
        "2024-01-01T10:35:00 | WARN | Low memory warning | memory.rs",
        "2024-01-01T11:00:00 | ERROR | Database connection failed | db.rs",
        "2024-01-01T11:05:00 | INFO | User login successful | auth.rs",
        "2024-01-01T11:30:00 | ERROR | File not found | file_handler.rs",
        "2024-01-01T12:00:00 | INFO | Scheduled task completed | scheduler.rs",
    ];
    
    let log_entries: Vec<LogEntry> = sample_logs.iter()
        .filter_map(|line| LogEntry::parse(line))
        .collect();
    
    let analytics = analyze_logs(log_entries);
    println!("Log Analytics: {:?}", analytics);
}

// 2. データ変換パイプライン
#[derive(Debug, Clone)]
struct RawData {
    id: String,
    value: String,
    category: String,
}

#[derive(Debug, Clone)]
struct ProcessedData {
    id: String,
    numeric_value: f64,
    category: String,
    normalized_value: f64,
}

fn validate_data(data: &RawData) -> Result<&RawData, String> {
    if data.id.is_empty() {
        Err("Empty ID".to_string())
    } else if data.value.is_empty() {
        Err("Empty value".to_string())
    } else {
        Ok(data)
    }
}

fn parse_numeric(data: &RawData) -> Result<f64, String> {
    data.value.parse::<f64>()
        .map_err(|_| format!("Invalid numeric value: {}", data.value))
}

fn normalize_value(value: f64, max_value: f64) -> f64 {
    if max_value == 0.0 {
        0.0
    } else {
        value / max_value
    }
}

fn process_data_pipeline(raw_data: Vec<RawData>) -> (Vec<ProcessedData>, Vec<String>) {
    // 最大値を計算（正規化のため）
    let max_value = raw_data.iter()
        .filter_map(|data| data.value.parse::<f64>().ok())
        .fold(0.0f64, |max, val| max.max(val));
    
    // データ処理パイプライン
    let (processed, errors): (Vec<_>, Vec<_>) = raw_data.iter()
        .map(|data| {
            validate_data(data)
                .and_then(|valid_data| {
                    parse_numeric(valid_data)
                        .map(|numeric_value| ProcessedData {
                            id: valid_data.id.clone(),
                            numeric_value,
                            category: valid_data.category.clone(),
                            normalized_value: normalize_value(numeric_value, max_value),
                        })
                })
        })
        .partition(|result| result.is_ok());
    
    let processed_data: Vec<ProcessedData> = processed.into_iter()
        .map(|result| result.unwrap())
        .collect();
    
    let error_messages: Vec<String> = errors.into_iter()
        .map(|result| result.unwrap_err())
        .collect();
    
    (processed_data, error_messages)
}

#[allow(unused)]
fn data_pipeline_example() {
    let raw_data = vec![
        RawData { id: "1".to_string(), value: "100".to_string(), category: "A".to_string() },
        RawData { id: "2".to_string(), value: "200".to_string(), category: "B".to_string() },
        RawData { id: "3".to_string(), value: "invalid".to_string(), category: "A".to_string() },
        RawData { id: "".to_string(), value: "150".to_string(), category: "C".to_string() },
        RawData { id: "4".to_string(), value: "75".to_string(), category: "B".to_string() },
    ];
    
    let (processed, errors) = process_data_pipeline(raw_data);
    
    println!("Processed data: {:?}", processed);
    println!("Errors: {:?}", errors);
}

// 3. 設定管理システム
#[derive(Debug, Clone)]
struct Config {
    database_url: String,
    port: u16,
    debug_mode: bool,
    max_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "localhost:5432".to_string(),
            port: 8080,
            debug_mode: false,
            max_connections: 100,
        }
    }
}

fn load_config_from_env() -> Result<Config, String> {
    let mut config = Config::default();
    
    // 環境変数から設定を読み込む関数型的なアプローチ
    type ConfigSetter = Box<dyn Fn(&mut Config, String) -> Result<(), &'static str>>;
    let env_mappings: Vec<(&str, ConfigSetter)> = vec![
        ("DATABASE_URL", Box::new(|config: &mut Config, value: String| {
            config.database_url = value;
            Ok(())
        })),
        ("PORT", Box::new(|config: &mut Config, value: String| {
            config.port = value.parse().map_err(|_| "Invalid port number")?;
            Ok(())
        })),
        ("DEBUG_MODE", Box::new(|config: &mut Config, value: String| {
            config.debug_mode = value.parse().map_err(|_| "Invalid boolean value")?;
            Ok(())
        })),
        ("MAX_CONNECTIONS", Box::new(|config: &mut Config, value: String| {
            config.max_connections = value.parse().map_err(|_| "Invalid number")?;
            Ok(())
        })),
    ];
    
    // 環境変数の処理
    for (key, setter) in env_mappings {
        if let Ok(value) = std::env::var(key) {
            setter(&mut config, value).map_err(|e| format!("Error setting {}: {}", key, e))?;
        }
    }
    
    Ok(config)
}

fn validate_config(config: &Config) -> Result<&Config, Vec<String>> {
    let mut errors = Vec::new();
    
    if config.port < 1024 {
        errors.push("Port should be >= 1024".to_string());
    }
    
    if config.max_connections == 0 {
        errors.push("Max connections should be > 0".to_string());
    }
    
    if config.database_url.is_empty() {
        errors.push("Database URL cannot be empty".to_string());
    }
    
    if errors.is_empty() {
        Ok(config)
    } else {
        Err(errors)
    }
}

#[allow(unused)]
fn config_management_example() {
    // 設定の読み込みと検証
    let config = load_config_from_env()
        .and_then(|config| {
            match validate_config(&config) {
                Ok(_) => Ok(config),
                Err(errors) => Err(errors.join(", "))
            }
        })
        .unwrap_or_else(|errors| {
            println!("Configuration errors: {}", errors);
            Config::default()
        });
    
    println!("Final config: {:?}", config);
}

// 4. キャッシュシステム
use std::hash::Hash;

struct Cache<K, V> {
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(max_size: usize) -> Self {
        Cache {
            data: HashMap::new(),
            max_size,
        }
    }
    
    fn get_or_compute<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.data.get(&key) {
            value.clone()
        } else {
            let value = compute();
            self.insert(key, value.clone());
            value
        }
    }
    
    fn insert(&mut self, key: K, value: V) {
        if self.data.len() >= self.max_size {
            // 簡単なLRU実装（実際にはもっと複雑）
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value);
    }
}

// 高コストな計算をシミュレート
fn expensive_computation(n: u32) -> u32 {
    // フィボナッチ数列の計算
    if n <= 1 {
        n
    } else {
        expensive_computation(n - 1) + expensive_computation(n - 2)
    }
}

#[allow(unused)]
fn cache_system_example() {
    let mut cache = Cache::new(10);
    
    // 関数型的なキャッシュ使用
    let numbers = vec![10, 5, 10, 8, 5, 12];
    
    let results: Vec<u32> = numbers.iter()
        .map(|&n| cache.get_or_compute(n, || expensive_computation(n)))
        .collect();
    
    println!("Results: {:?}", results);
}

// 5. バリデーションシステム
#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
    age: u32,
}

type ValidationResult<T> = Result<T, Vec<String>>;

fn validate_name(name: &str) -> ValidationResult<String> {
    if name.is_empty() {
        Err(vec!["Name cannot be empty".to_string()])
    } else if name.len() < 2 {
        Err(vec!["Name must be at least 2 characters".to_string()])
    } else {
        Ok(name.to_string())
    }
}

fn validate_email(email: &str) -> ValidationResult<String> {
    if email.is_empty() {
        Err(vec!["Email cannot be empty".to_string()])
    } else if !email.contains('@') {
        Err(vec!["Invalid email format".to_string()])
    } else {
        Ok(email.to_string())
    }
}

fn validate_age(age: u32) -> ValidationResult<u32> {
    if age < 13 {
        Err(vec!["Age must be at least 13".to_string()])
    } else if age > 120 {
        Err(vec!["Age must be less than 120".to_string()])
    } else {
        Ok(age)
    }
}

fn validate_user(name: &str, email: &str, age: u32) -> ValidationResult<User> {
    let name_result = validate_name(name);
    let email_result = validate_email(email);
    let age_result = validate_age(age);
    
    // 全てのバリデーション結果を組み合わせ
    match (name_result, email_result, age_result) {
        (Ok(name), Ok(email), Ok(age)) => Ok(User { name, email, age }),
        _ => {
            let mut errors = Vec::new();
            
            if let Err(mut name_errors) = validate_name(name) {
                errors.append(&mut name_errors);
            }
            if let Err(mut email_errors) = validate_email(email) {
                errors.append(&mut email_errors);
            }
            if let Err(mut age_errors) = validate_age(age) {
                errors.append(&mut age_errors);
            }
            
            Err(errors)
        }
    }
}

#[allow(unused)]
fn validation_system_example() {
    let test_users = vec![
        ("Alice", "alice@example.com", 25),
        ("", "bob@example.com", 30),
        ("Charlie", "invalid-email", 20),
        ("David", "david@example.com", 10),
    ];
    
    let results: Vec<ValidationResult<User>> = test_users.iter()
        .map(|(name, email, age)| validate_user(name, email, *age))
        .collect();
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(user) => println!("User {}: {:?}", i + 1, user),
            Err(errors) => println!("User {} errors: {:?}", i + 1, errors),
        }
    }
}

// 6. イベント処理システム
#[derive(Debug, Clone)]
enum Event {
    UserRegistered { user_id: String, email: String },
    UserLoggedIn { user_id: String },
    OrderPlaced { user_id: String, order_id: String, amount: f64 },
    PaymentProcessed { order_id: String, amount: f64 },
}

#[derive(Debug, Clone)]
struct EventStats {
    total_events: usize,
    registrations: usize,
    logins: usize,
    orders: usize,
    payments: usize,
    total_revenue: f64,
}

fn process_events(events: Vec<Event>) -> EventStats {
    events.iter().fold(
        EventStats {
            total_events: 0,
            registrations: 0,
            logins: 0,
            orders: 0,
            payments: 0,
            total_revenue: 0.0,
        },
        |mut stats, event| {
            stats.total_events += 1;
            
            match event {
                Event::UserRegistered { .. } => stats.registrations += 1,
                Event::UserLoggedIn { .. } => stats.logins += 1,
                Event::OrderPlaced { amount, .. } => {
                    stats.orders += 1;
                    stats.total_revenue += amount;
                }
                Event::PaymentProcessed { .. } => stats.payments += 1,
            }
            
            stats
        },
    )
}

fn generate_user_insights(events: Vec<Event>) -> HashMap<String, usize> {
    events.iter()
        .filter_map(|event| match event {
            Event::UserRegistered { user_id, .. } => Some((user_id.clone(), "registration")),
            Event::UserLoggedIn { user_id } => Some((user_id.clone(), "login")),
            Event::OrderPlaced { user_id, .. } => Some((user_id.clone(), "order")),
            _ => None,
        })
        .fold(HashMap::new(), |mut acc, (user_id, event_type)| {
            let key = format!("{}_{}", user_id, event_type);
            *acc.entry(key).or_insert(0) += 1;
            acc
        })
}

#[allow(unused)]
fn event_processing_example() {
    let events = vec![
        Event::UserRegistered { user_id: "user1".to_string(), email: "user1@example.com".to_string() },
        Event::UserLoggedIn { user_id: "user1".to_string() },
        Event::OrderPlaced { user_id: "user1".to_string(), order_id: "order1".to_string(), amount: 99.99 },
        Event::PaymentProcessed { order_id: "order1".to_string(), amount: 99.99 },
        Event::UserRegistered { user_id: "user2".to_string(), email: "user2@example.com".to_string() },
        Event::UserLoggedIn { user_id: "user2".to_string() },
        Event::OrderPlaced { user_id: "user2".to_string(), order_id: "order2".to_string(), amount: 149.99 },
    ];
    
    let stats = process_events(events.clone());
    println!("Event stats: {:?}", stats);
    
    let user_insights = generate_user_insights(events);
    println!("User insights: {:?}", user_insights);
}

// 7. 機能フラグシステム
#[derive(Debug, Clone)]
struct FeatureFlag {
    name: String,
    enabled: bool,
    rollout_percentage: f64,
}

#[derive(Debug)]
struct FeatureFlags {
    flags: HashMap<String, FeatureFlag>,
}

impl FeatureFlags {
    fn new() -> Self {
        FeatureFlags {
            flags: HashMap::new(),
        }
    }
    
    fn add_flag(&mut self, name: String, enabled: bool, rollout_percentage: f64) {
        self.flags.insert(name.clone(), FeatureFlag {
            name,
            enabled,
            rollout_percentage,
        });
    }
    
    fn is_enabled(&self, flag_name: &str, user_id: &str) -> bool {
        self.flags.get(flag_name)
            .map(|flag| {
                if !flag.enabled {
                    return false;
                }
                
                // 簡単なハッシュベースの判定
                let hash = user_id.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
                let percentage = (hash % 100) as f64;
                percentage < flag.rollout_percentage
            })
            .unwrap_or(false)
    }
}

#[allow(unused)]
fn feature_flag_example() {
    let mut flags = FeatureFlags::new();
    flags.add_flag("new_ui".to_string(), true, 50.0);
    flags.add_flag("beta_feature".to_string(), true, 10.0);
    flags.add_flag("disabled_feature".to_string(), false, 100.0);
    
    let test_users = vec!["user1", "user2", "user3", "user4", "user5"];
    
    for user in test_users {
        let new_ui_enabled = flags.is_enabled("new_ui", user);
        let beta_enabled = flags.is_enabled("beta_feature", user);
        let disabled_enabled = flags.is_enabled("disabled_feature", user);
        
        println!("User {}: new_ui={}, beta={}, disabled={}", 
                 user, new_ui_enabled, beta_enabled, disabled_enabled);
    }
}

// メイン関数
fn main() {
    println!("=== Rust 関数型プログラミング 実践編 ===\n");
    
    println!("1. ログ解析システム:");
    log_analysis_example();
    
    println!("\n2. データ変換パイプライン:");
    data_pipeline_example();
    
    println!("\n3. 設定管理システム:");
    config_management_example();
    
    println!("\n4. キャッシュシステム:");
    cache_system_example();
    
    println!("\n5. バリデーションシステム:");
    validation_system_example();
    
    println!("\n6. イベント処理システム:");
    event_processing_example();
    
    println!("\n7. 機能フラグシステム:");
    feature_flag_example();
}

// 練習問題とテスト
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_parsing() {
        let log_line = "2024-01-01T10:30:00 | INFO | Test message | test.rs";
        let entry = LogEntry::parse(log_line).unwrap();
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "Test message");
    }
    
    #[test]
    fn test_data_validation() {
        let valid_data = RawData {
            id: "test".to_string(),
            value: "123".to_string(),
            category: "A".to_string(),
        };
        
        assert!(validate_data(&valid_data).is_ok());
        
        let invalid_data = RawData {
            id: "".to_string(),
            value: "123".to_string(),
            category: "A".to_string(),
        };
        
        assert!(validate_data(&invalid_data).is_err());
    }
    
    #[test]
    fn test_user_validation() {
        let valid_user = validate_user("Alice", "alice@example.com", 25);
        assert!(valid_user.is_ok());
        
        let invalid_user = validate_user("", "invalid-email", 10);
        assert!(invalid_user.is_err());
        
        if let Err(errors) = invalid_user {
            assert!(errors.len() > 0);
        }
    }
    
    #[test]
    fn test_event_processing() {
        let events = vec![
            Event::UserRegistered { user_id: "user1".to_string(), email: "test@example.com".to_string() },
            Event::OrderPlaced { user_id: "user1".to_string(), order_id: "order1".to_string(), amount: 100.0 },
        ];
        
        let stats = process_events(events);
        assert_eq!(stats.total_events, 2);
        assert_eq!(stats.registrations, 1);
        assert_eq!(stats.orders, 1);
        assert_eq!(stats.total_revenue, 100.0);
    }
    
    #[test]
    fn test_cache_system() {
        let mut cache = Cache::new(2);
        
        // 初回は計算される
        let result1 = cache.get_or_compute("key1", || 42);
        assert_eq!(result1, 42);
        
        // 2回目はキャッシュから取得
        let result2 = cache.get_or_compute("key1", || 100);
        assert_eq!(result2, 42); // キャッシュされた値
    }
}