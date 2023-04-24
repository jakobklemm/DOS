//! # System Info state

use serde::Serialize;

use crate::error::TABError;

#[derive(Clone, Debug, Serialize)]
pub struct System(pub Vec<Element>);

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Element {
    package: String,
    name: String,
    damage: String,
    count: String,
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cint: usize = self.count.parse().unwrap();
        let oint: usize = other.count.parse().unwrap();
        cint.partial_cmp(&oint)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cint: usize = self.count.parse().unwrap();
        let oint: usize = other.count.parse().unwrap();
        cint.cmp(&oint)
    }
}

impl System {
    pub fn push(&mut self, e: Element) {
        if let Some(i) = self.0.iter().position(|r| r.name() == e.name()) {
            self.0.get_mut(i).unwrap().count = e.count;
        } else {
            self.0.push(e);
        }
    }

    pub fn parse(&self) -> String {
        // let dict = Dictionary::load();

        let mut sum = String::from("# HELP tab_me Item in the ME system\n");

        for element in self.0.clone().into_iter() {
            /* if let Some(name) = dict.list.get(&element.name()) {
                let ret = format!("tab_me_{} = {}\n", name, element.count);

                sum += &ret;
            }
            */
            let ret = format!("tab_me_{} {}\n", element.name(), element.count);

            let ret = ret.replace("-", "_");
            let ret = ret.replace(".", "_");

            sum += &ret;
        }

        sum += &format!("tab_me_total_count {}", self.0.len());

        sum
    }
}

impl Element {
    pub fn name(&self) -> String {
        format!("{}_{}_{}", self.package, self.name, self.damage)
    }

    pub fn from(source: String) -> Result<Self, TABError> {
        let elements: Vec<&str> = source.split(":").collect();

        if elements.len() < 4 {
            return Err(TABError::default());
        }

        let package = elements.get(0).ok_or(TABError::default())?;
        let name = elements.get(1).ok_or(TABError::default())?;
        let damage = elements.get(2).ok_or(TABError::default())?;
        let count = elements.get(3).ok_or(TABError::default())?;

        Ok(Element {
            package: package.to_string(),
            name: name.to_string(),
            damage: damage.to_string(),
            count: count.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_element() {
        let source = String::from("ic2:misc_resource:3:512");
        let e = Element::from(source).unwrap();
        let target = Element {
            package: String::from("ic2"),
            name: String::from("misc_resource"),
            damage: String::from("3"),
            count: String::from("512"),
        };
        assert_eq!(e, target);
    }

    #[test]
    fn test_parse_hbm() {
        let source = String::from("hbm:powder_polonium:0:64");
        let e = Element::from(source).unwrap();
        let target = Element {
            package: String::from("hbm"),
            name: String::from("powder_polonium"),
            damage: String::from("0"),
            count: String::from("64"),
        };
        assert_eq!(e, target);
    }
}
