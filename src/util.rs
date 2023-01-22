use std::collections::HashMap;

use crate::*;
use rand::{thread_rng, Rng};

/// Generates a vec that has a very specific set of information relative to a lifeform, to be used
/// later on with the close_lifeform_info_from_info_vec function
pub fn generate_lifeform_info_vec(
    lifeforms: &HashMap<usize, LifeForm>,
) -> Vec<(usize, (usize, usize), f32)> {
    lifeforms
        .values()
        .map(|lifeform| (lifeform.id, lifeform.location, lifeform.health))
        .collect()
}

/// Takes id and location of the thing you're trying to find the closest other thing to, very
/// specificly constructed vector
/// Returns (
///     num_in_vicinity, (number of lifeforms within the vicinity of the lifeform)
///     health,  (of closest lf)
///     loc, (of closest lf)
///     distance (of closest lf)
/// )
pub fn close_lifeform_info_from_info_vec(
    size: usize,
    id: &usize,
    location: &(usize, usize),
    lfs_id_loc_health: &Vec<(usize, (usize, usize), f32)>,
) -> (usize, f32, (usize, usize), f32) {
    let mut number_in_vicinity: usize = 0;
    let mut closest_lf_health: f32 = 0.0;
    let mut closest_lf_location: (usize, usize) = (0, 0);
    let mut closest_lf_distance = f32::INFINITY;
    for (object_id, loc, health) in lfs_id_loc_health {
        if object_id == id {
            continue;
        }

        let dist = dist_rel(size, location, loc);

        if dist < closest_lf_distance {
            closest_lf_health = *health;
            closest_lf_distance = dist;
            closest_lf_location = *loc;
        }

        if dist < 2.0 {
            number_in_vicinity += 1;
        }
    }

    (
        number_in_vicinity,
        closest_lf_health,
        closest_lf_location,
        closest_lf_distance,
    )
}

pub fn closest_to(subject: &(usize, usize), objects: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut shortest_distance = f32::INFINITY;
    let mut closest_object = (0, 0);

    for object in objects {
        let distance = dist_abs(subject, object);
        if distance < shortest_distance {
            shortest_distance = distance;
            closest_object = *object;
        }
    }

    closest_object
}

/// The distance from one point to another
pub fn dist_abs(from: &(usize, usize), to: &(usize, usize)) -> f32 {
    let x1 = from.0 as f32;
    let y1 = from.1 as f32;
    let x2 = to.0 as f32;
    let y2 = to.1 as f32;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Gives a distance to an object _relative to the size of the world_. This is to be used for input
/// neuron values, which need to be between 0 and 1. The maximum value you can get is from one
/// corner of the world to the opposite corner, that will be of distance 1.0. Two points on top of
/// each other will be 0.0.
pub fn dist_rel(world_size: usize, from: &(usize, usize), to: &(usize, usize)) -> f32 {
    let farthest_possible = ((2 * world_size.pow(2)) as f32).sqrt();
    dist_abs(from, to) / farthest_possible
}

/// Returns one of the four cardinal directions, and returns north if it's the same point.
/// And assumes that the start point is in the bottom left corner.
pub fn direc(from: &(usize, usize), to: &(usize, usize)) -> DirectionName {
    let x1 = from.0 as f32;
    let y1 = from.1 as f32;
    let x2 = to.0 as f32;
    let y2 = to.1 as f32;

    if y2 > y1 && y2 + y1 > x2 + x1 {
        return DirectionName::North;
    } else if x2 > x1 && x2 + x1 > y2 + y1 {
        return DirectionName::East;
    } else if y2 < y1 && y2 + y1 > x2 + x1 {
        return DirectionName::South;
    } else if x2 < x1 && x2 + x1 > y2 + y1 {
        return DirectionName::West;
    }

    if x2 == x1 {
        if y2 < y1 {
            // straight down
            return DirectionName::South;
        } else if y2 > y1 {
            // straight up
            return DirectionName::North;
        }
    }

    if y2 == y1 {
        if x2 > x1 {
            // straight right
            return DirectionName::East;
        } else if x2 < x1 {
            // straight left
            return DirectionName::West;
        }
    }

    // Otherwise it's the same point
    DirectionName::North
}

/// Relocate one step randomly
pub fn randomize(size: usize, mut loc: &mut (usize, usize)) {
    if loc.0 == 0 {
        loc.0 = 1;
        return;
    } else if loc.0 == size {
        loc.0 = size - 1;
        return;
    } else if loc.1 == 0 {
        loc.1 = 1;
        return;
    } else if loc.1 == size {
        loc.1 = size - 1;
        return;
    }

    if thread_rng().gen_bool(0.5) {
        if thread_rng().gen_bool(0.5) {
            loc.0 += 1;
            return;
        } else {
            loc.0 -= 1;
            return;
        }
    } else {
        if thread_rng().gen_bool(0.5) {
            loc.1 += 1;
            return;
        } else {
            loc.1 -= 1;
            return;
        }
    }
}

/// Helper to abstract and test the math of movement, not being able to go over edges especially.
pub fn update_location(size: usize, loc: &mut (usize, usize), modifier: &(i8, i8)) {
    let xm = modifier.0 as isize;
    let ym = modifier.1 as isize;
    let x = loc.0 as isize;
    let y = loc.1 as isize;

    let mut xn = x + xm;
    let mut yn = y + ym;

    if xn < 0 {
        xn = 0;
    }

    if xn > size as isize {
        xn = size as isize;
    }

    if yn < 0 {
        yn = 0;
    }

    if yn > size as isize {
        yn = size as isize;
    }

    loc.0 = xn as usize;
    loc.1 = yn as usize;
}

/// Gives a direction relative to a location and an orientation. So if the thing is oriented directly
/// northwards, and the other_location given is due north of the self_location, this will return
/// straight ahead.
/// 0: straight ahead
/// -0.5: due left
/// 0.5: Due right
/// 1: Straight back
pub fn rel_dir(
    self_location: &(usize, usize),
    orientation: &(i8, i8),
    other_location: &(usize, usize),
) -> f32 {
    // Gotta handle when it's the same point
    if self_location == other_location {
        return 0.0;
    }

    // self
    let xs = self_location.0 as f32;
    let ys = self_location.1 as f32;

    // self plus orientation
    let xsor = xs + orientation.0 as f32;
    let ysor = ys + orientation.1 as f32;

    // other
    let xo = other_location.0 as f32;
    let yo = other_location.1 as f32;

    // Great formula from https://stackoverflow.com/a/31334882/2128027

    // result = atan2(P3.y - P1.y, P3.x - P1.x) -
    //          atan2(P2.y - P1.y, P2.x - P1.x);

    // P1 is self, P2 is self plus or, P3 is other
    // result = atan2(yo - ys, xo - xs) -
    //          atan2(ysor - ys, xsor - xs);

    let mut radians = (yo - ys).atan2(xo - xs) - (ysor - ys).atan2(xsor - xs);

    // We don't care about obtuse angles, this translates them to their acute brethren
    if radians > std::f32::consts::PI {
        radians -= std::f32::consts::PI * 2.0;
    } else if radians < -std::f32::consts::PI {
        radians += std::f32::consts::PI * 2.0;
    }

    // Now convert to a unit digit
    radians / std::f32::consts::PI
}

/// Takes a mutable subject and moves it one step towards a given object, each being a location
/// TODO Currently only does cardinal directions but could also be updated to get all 8.
pub fn move_towards(size: usize, subject: &mut (usize, usize), object: &(usize, usize)) {
    match direc(subject, object) {
        DirectionName::North => update_location(size, subject, &(0, 1)),
        DirectionName::East => update_location(size, subject, &(1, 0)),
        DirectionName::South => update_location(size, subject, &(0, -1)),
        DirectionName::West => update_location(size, subject, &(-1, 0)),
        _ => ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_dir() {

        let self_location = (2, 2);
        let orientation = (1, 1);
        let other_location = (2, 2);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 0.0);

        let self_location = (2, 2);
        let orientation = (0, 1);
        let other_location = (2, 3);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 0.0);

        let self_location = (2, 2);
        let orientation = (0, -1);
        let other_location = (2, 3);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 1.0);

        let self_location = (2, 2);
        let orientation = (0, 1);
        let other_location = (1, 1);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 0.75);

        let self_location = (2, 2);
        let orientation = (0, 1);
        let other_location = (3, 1);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, -0.75);

        let self_location = (2, 2);
        let orientation = (1, 1);
        let other_location = (3, 3);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 0.0);

        let self_location = (2, 2);
        let orientation = (0, 1);
        let other_location = (1, 2);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 0.5);

        let self_location = (2, 2);
        let orientation = (0, 1);
        let other_location = (3, 2);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, -0.5);

        let self_location = (2, 2);
        let orientation = (-1, 1);
        let other_location = (1, 3);

        let dir = rel_dir(&self_location, &orientation, &other_location);
        assert_eq!(dir, 0.0);
    }

    #[test]
    fn test_update_location() {
        let mut loc = (5, 5);
        update_location(100, &mut loc, &(0, 0));
        assert_eq!(loc, (5, 5));

        let mut loc = (5, 5);
        update_location(100, &mut loc, &(1, 1));
        assert_eq!(loc, (6, 6));

        let mut loc = (1, 1);
        update_location(100, &mut loc, &(-1, -1));
        assert_eq!(loc, (0, 0));

        let mut loc = (0, 0);
        update_location(100, &mut loc, &(-1, -1));
        assert_eq!(loc, (0, 0));

        let mut loc = (1, 1);
        update_location(1, &mut loc, &(1, 1));
        assert_eq!(loc, (1, 1));
    }

    #[test]
    fn test_closest_to() {
        let subject = (0, 0);
        let objects = vec![(1, 1), (2, 2), (3, 3)];

        let loc = closest_to(&subject, &objects);
        assert_eq!(objects[0], loc);

        let subject = (5, 5);
        let objects = vec![(1, 1), (2, 2), (3, 3)];

        let loc = closest_to(&subject, &objects);
        assert_eq!(objects[2], loc);

        let subject = (2, 2);
        let objects = vec![(1, 1), (2, 2), (3, 3)];

        let loc = closest_to(&subject, &objects);
        assert_eq!(objects[1], loc);
    }

    #[test]
    fn test_dist_rel() {
        let l1 = (0, 0);
        let l2 = (0, 0);
        assert_eq!(dist_rel(10, &l1, &l2), 0.0);

        let l1 = (5, 5);
        let l2 = (5, 5);
        assert_eq!(dist_rel(10, &l1, &l2), 0.0);

        let l1 = (1, 0);
        let l2 = (0, 1);
        assert_eq!(dist_rel(10, &l1, &l2), 0.1);

        let l1 = (0, 1);
        let l2 = (1, 0);
        assert_eq!(dist_rel(10, &l1, &l2), 0.1);

        let l1 = (10, 0);
        let l2 = (0, 10);
        assert_eq!(dist_rel(10, &l1, &l2), 1.0);

        let l1 = (0, 10);
        let l2 = (10, 0);
        assert_eq!(dist_rel(10, &l1, &l2), 1.0);
    }

    #[test]
    fn test_dist_abs() {
        let l1 = (0, 0);
        let l2 = (0, 0);
        assert_eq!(dist_abs(&l1, &l2), 0.0);

        let l1 = (5, 5);
        let l2 = (5, 5);
        assert_eq!(dist_abs(&l1, &l2), 0.0);

        let l1 = (1, 0);
        let l2 = (0, 0);
        assert_eq!(dist_abs(&l1, &l2), 1.0);

        let l1 = (2, 0);
        let l2 = (0, 0);
        assert_eq!(dist_abs(&l1, &l2), 2.0);

        let l1 = (0, 1);
        let l2 = (0, 0);
        assert_eq!(dist_abs(&l1, &l2), 1.0);

        let l1 = (0, 2);
        let l2 = (0, 0);
        assert_eq!(dist_abs(&l1, &l2), 2.0);

        let l1 = (0, 0);
        let l2 = (1, 0);
        assert_eq!(dist_abs(&l1, &l2), 1.0);

        let l1 = (0, 0);
        let l2 = (2, 0);
        assert_eq!(dist_abs(&l1, &l2), 2.0);

        let l1 = (0, 1);
        let l2 = (1, 0);
        assert_eq!(dist_abs(&l1, &l2), (2.0 as f32).sqrt());

        let l1 = (1, 0);
        let l2 = (0, 1);
        assert_eq!(dist_abs(&l1, &l2), (2.0 as f32).sqrt());
    }

    #[test]
    fn test_direc() {
        // Same point
        let l1 = (0, 0);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), DirectionName::North);

        let l1 = (5, 5);
        let l2 = (5, 5);
        assert_eq!(direc(&l1, &l2), DirectionName::North);

        // Generally north
        let l1 = (0, 0);
        let l2 = (0, 1);
        assert_eq!(direc(&l1, &l2), DirectionName::North);

        let l1 = (0, 0);
        let l2 = (1, 3);
        assert_eq!(direc(&l1, &l2), DirectionName::North);

        // Generally east
        let l1 = (0, 0);
        let l2 = (1, 0);
        assert_eq!(direc(&l1, &l2), DirectionName::East);

        let l1 = (0, 1);
        let l2 = (5, 2);
        assert_eq!(direc(&l1, &l2), DirectionName::East);

        // Generally south
        let l1 = (0, 1);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), DirectionName::South);

        let l1 = (1, 2);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), DirectionName::South);

        // Generally west
        let l1 = (1, 0);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), DirectionName::West);

        let l1 = (3, 2);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), DirectionName::West);
    }

    #[test]
    fn test_randomize() {
        for _ in 0..100 {
            let mut loc = (5, 5);
            randomize(10, &mut loc);
            assert_ne!(loc, (5, 5));

            // It has to move by only one though
            assert!([4,5,6].contains(&loc.0));
            assert!([4,5,6].contains(&loc.1));
        }
    }
}
