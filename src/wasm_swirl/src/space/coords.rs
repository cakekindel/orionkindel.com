pub trait Perpendicular<Cmp = Self> {
  fn perpendicular(&self, cmp: &Cmp) -> bool;
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub enum Axis {
  X,
  Y,
}
impl Perpendicular for Axis {
  fn perpendicular(&self, cmp: &Self) -> bool {
    self != cmp
  }
}

impl Perpendicular<Axis> for Edge {
  fn perpendicular(&self, cmp: &Axis) -> bool {
    self.as_axis().perpendicular(cmp)
  }
}

impl Perpendicular for Edge {
  fn perpendicular(&self, cmp: &Self) -> bool {
    self.as_axis().perpendicular(&cmp.as_axis())
  }
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub enum EdgeInfo {
  Corner(Edge, Edge),
  Edge(Edge),
  Neither,
}

impl EdgeInfo {
  pub fn get_edge(&self) -> Option<Edge> {
    match self {
      Self::Edge(edge) => Some(*edge),
      _ => None,
    }
  }
}

impl From<(Option<Edge>, Option<Edge>)> for EdgeInfo {
  fn from(edges: (Option<Edge>, Option<Edge>)) -> Self {
    use Edge::*;

    // this is dumb.
    match edges {
      (None, None) => Self::Neither,
      (Some(Top), Some(Left)) | (Some(Left), Some(Top)) => Self::Corner(Top, Left),
      (Some(Top), Some(Right)) | (Some(Right), Some(Top)) => Self::Corner(Top, Right),
      (Some(Bottom), Some(Right)) | (Some(Right), Some(Bottom)) => Self::Corner(Bottom, Right),
      (Some(Bottom), Some(Left)) | (Some(Left), Some(Bottom)) => Self::Corner(Bottom, Left),
      (Some(Top), _) | (_, Some(Top)) => Self::Edge(Top),
      (Some(Bottom), _) | (_, Some(Bottom)) => Self::Edge(Bottom),
      (Some(Left), _) | (_, Some(Left)) => Self::Edge(Left),
      (Some(Right), _) | (_, Some(Right)) => Self::Edge(Right),
    }
  }
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub enum Edge {
  Top,
  Bottom,
  Left,
  Right,
}
impl Edge {
  pub fn as_axis(&self) -> Axis {
    use Axis::*;
    use Edge::*;

    match self {
      Top | Bottom => Y,
      Left | Right => X,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Corner(pub Edge, pub Edge);
impl Corner {
  pub fn all() -> [Self; 4] {
    use Edge::*;
    [
      Self(Top, Left),
      Self(Top, Right),
      Self(Bottom, Left),
      Self(Bottom, Right),
    ]
  }
}

#[derive(Debug, Hash, Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct Coord2 {
  pub x: usize,
  pub y: usize,
}

impl From<(usize, usize)> for Coord2 {
  fn from((x, y): (usize, usize)) -> Self {
    Coord2 { x, y }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
  pub height: usize,
  pub width: usize,
  pub origin: Coord2,
}

impl Rect {
  /// Test whether a coordinate falls within the bounds
  /// of the Rect
  pub fn contains(&self, coords: impl Into<Coord2>) -> bool {
    let coords = coords.into();
    let x_bound_lower = self.origin.x;
    let x_bound_upper = self.origin.x + self.width;
    let x_good = coords.x >= x_bound_lower && coords.x <= x_bound_upper;

    let y_bound_lower = self.origin.y;
    let y_bound_upper = self.origin.y + self.height;
    let y_good = coords.y >= y_bound_lower && coords.y <= y_bound_upper;

    x_good && y_good
  }

  /// Returns the area of the Rect
  pub fn area(&self) -> usize {
    self.height * self.width
  }

  /// Get an iterator over the coordinates of the edges and corners
  /// of the Rect
  pub fn edge_coord_iter<'a>(&'a self) -> impl Iterator<Item = (EdgeInfo, Coord2)> {
    let max_x = self.width - 1;
    let max_y = self.height - 1;
    let rect = *self;

    let top_edge = (0..=max_x).map(move |x| (x, max_y));

    let bot_edge = (0..=max_x).map(|x| (x, 0));

    // exclude corners from left and right edges,
    // because they're already added by top and bottom edges
    let left_edge = (1..max_y).map(|y| (0, y));

    let right_edge = (1..max_y).map(move |y| (max_x, y));

    top_edge
      .chain(bot_edge)
      .chain(left_edge)
      .chain(right_edge)
      .map(Coord2::from)
      .map(move |coords| (rect.get_edge_info(coords), Coord2::from(coords)))
  }

  /// Given a coordinate, get the information about whether
  /// that point resides on:
  /// - an edge of the grid
  /// - a corner of the grid
  /// - neither
  pub fn get_edge_info(&self, coords: Coord2) -> EdgeInfo {
    let max_x = self.width - 1;
    let max_y = self.height - 1;

    let left_edge = if coords.x == 0 {
      Some(Edge::Left)
    } else if coords.x == max_x {
      Some(Edge::Right)
    } else {
      None
    };
    let right_edge = if coords.y == 0 {
      Some(Edge::Bottom)
    } else if coords.y == max_y {
      Some(Edge::Top)
    } else {
      None
    };

    (left_edge, right_edge).into()
  }
}
