// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-boot-performance: boot performance component model.
//!
//! Real implementation: performances, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Performance {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootPerformanceError {
    EmptyPerformanceName,
    DuplicatePerformance(String),
    EmptyEventSubject,
}

impl fmt::Display for BootPerformanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPerformanceName => write!(f, "performance name must not be empty"),
            Self::DuplicatePerformance(id) => write!(f, "duplicate performance {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BootPerformanceError {}

pub struct BootPerformance {
    pub name: String,
    performances: BTreeMap<String, Performance>,
    events: BTreeMap<String, Event>,
}

impl BootPerformance {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            performances: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_performance(&mut self, item: Performance) -> Result<(), BootPerformanceError> {
        if item.name.trim().is_empty() {
            return Err(BootPerformanceError::EmptyPerformanceName);
        }
        let id = item.id.clone();
        if self.performances.contains_key(&id) {
            return Err(BootPerformanceError::DuplicatePerformance(id.clone()));
        }
        self.performances.insert(id, item);
        Ok(())
    }

    pub fn performance_count(&self) -> usize {
        self.performances.len()
    }

    pub fn get_performance(&self, id: &str) -> Option<&Performance> {
        self.performances.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BootPerformanceError> {
        if event.subject.trim().is_empty() {
            return Err(BootPerformanceError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_performance(id: &str, name: &str) -> Performance {
        Performance {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = BootPerformance::new("BootPerformance");
        assert_eq!(m.name, "BootPerformance");
        assert_eq!(m.performance_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_performance() {
        let mut m = BootPerformance::new("BootPerformance");
        m.add_performance(sample_performance("a1", "Alpha")).unwrap();
        assert_eq!(m.performance_count(), 1);
        assert!(m.get_performance("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = BootPerformance::new("BootPerformance");
        assert!(m.add_performance(sample_performance("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = BootPerformance::new("BootPerformance");
        m.add_performance(sample_performance("x", "One")).unwrap();
        assert!(m.add_performance(sample_performance("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = BootPerformance::new("BootPerformance");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
