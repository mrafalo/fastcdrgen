use crate::customer::Customer;

#[derive(Clone, Debug)]
pub enum RelationType {
    FRIEND,
    FAMILY,
    BUSINESS,
    OTHER,
}


#[derive(Clone, Debug)]
pub struct Relation{
    pub from: Customer,
    pub to: Customer,
    pub relation_type: RelationType

}
