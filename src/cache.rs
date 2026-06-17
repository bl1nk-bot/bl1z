use crate::ast::SpannedExpr;
use std::collections::{HashMap, VecDeque};

/// LRU-style cache for parsed formula expressions.
///
/// Maps formula source strings to their parsed AST representations,
/// avoiding redundant re-parsing of the same formula.
///
/// # Example
///
/// ```rust
/// use bl1z::{tokenize, parse};
/// use bl1z::cache::FormulaCache;
///
/// let mut cache = FormulaCache::new(100);
///
/// // Cache miss — parse and store
/// let tokens = tokenize("1 + 2").unwrap();
/// let ast = parse(&tokens).unwrap();
/// cache.insert("1 + 2".to_string(), ast.clone());
///
/// // Cache hit — skip parsing
/// assert!(cache.get("1 + 2").is_some());
/// assert_eq!(cache.len(), 1);
/// ```
pub struct FormulaCache {
    entries: HashMap<String, SpannedExpr>,
    order: VecDeque<String>,
    capacity: usize,
}

impl FormulaCache {
    /// Creates a new cache with the given capacity.
    ///
    /// When the cache is full and a new entry is added, the oldest
    /// entry is evicted (FIFO approximation of LRU).
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity.min(1024)),
            order: VecDeque::with_capacity(capacity.min(1024)),
            capacity,
        }
    }

    /// Creates a cache with a default capacity of 256.
    pub fn with_default_capacity() -> Self {
        Self::new(256)
    }

    /// Inserts a formula and its parsed AST into the cache.
    ///
    /// If the cache is at capacity, the oldest entry is evicted.
    #[allow(clippy::map_entry)]
    pub fn insert(&mut self, formula: String, ast: SpannedExpr) {
        if self.entries.contains_key(&formula) {
            // Update existing entry (no eviction needed)
            self.entries.insert(formula, ast);
            return;
        }
        if self.entries.len() >= self.capacity {
            // Evict oldest entry
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            }
        }
        self.order.push_back(formula.clone());
        self.entries.insert(formula, ast);
    }

    /// Retrieves the cached AST for a given formula string.
    pub fn get(&self, formula: &str) -> Option<&SpannedExpr> {
        self.entries.get(formula)
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the maximum capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clears all entries from the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }

    /// Removes a specific formula from the cache.
    pub fn invalidate(&mut self, formula: &str) -> bool {
        if self.entries.remove(formula).is_some() {
            self.order.retain(|k| k != formula);
            true
        } else {
            false
        }
    }
}

impl Default for FormulaCache {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn make_ast(formula: &str) -> SpannedExpr {
        let tokens = tokenize(formula).expect("tokenize failed");
        parse(&tokens).expect("parse failed")
    }

    #[test]
    fn cache_miss_and_hit() {
        let mut cache = FormulaCache::new(10);
        let ast = make_ast("1 + 2");
        cache.insert("1 + 2".to_string(), ast.clone());
        let cached = cache.get("1 + 2");
        assert!(cached.is_some());
    }

    #[test]
    fn cache_capacity_eviction() {
        let mut cache = FormulaCache::new(2);
        cache.insert("a".to_string(), make_ast("1"));
        cache.insert("b".to_string(), make_ast("2"));
        cache.insert("c".to_string(), make_ast("3"));
        assert_eq!(cache.len(), 2);
        // "a" should have been evicted
        assert!(cache.get("a").is_none());
        assert!(cache.get("b").is_some());
        assert!(cache.get("c").is_some());
    }

    #[test]
    fn cache_invalidate() {
        let mut cache = FormulaCache::new(10);
        cache.insert("x".to_string(), make_ast("42"));
        assert!(cache.invalidate("x"));
        assert!(cache.get("x").is_none());
        assert!(!cache.invalidate("x"));
    }

    #[test]
    fn cache_clear() {
        let mut cache = FormulaCache::new(10);
        cache.insert("a".to_string(), make_ast("1"));
        cache.insert("b".to_string(), make_ast("2"));
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn cache_default_capacity() {
        let cache = FormulaCache::default();
        assert_eq!(cache.capacity(), 256);
        assert!(cache.is_empty());
    }

    #[test]
    fn cache_replace_same_key() {
        let mut cache = FormulaCache::new(10);
        cache.insert("x".to_string(), make_ast("1"));
        cache.insert("x".to_string(), make_ast("2"));
        assert_eq!(cache.len(), 1);
    }
}
