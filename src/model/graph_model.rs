use crate::views::Point;

pub trait HealthGraphModel {
    fn to_points() -> Vec<Point>;
    fn reload() -> bool;
}