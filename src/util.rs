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

/// Returns
/// 0.25 for north
/// 0.50 for east
/// 0.75 for south
/// 1.00 for west
/// 0.00 for same point
/// And assumes that the start point is in the upper left corner, like most terminals represent it
/// as. Note that with tui-rs, the visual library used in this project, it actually represents the
/// bottom left as (0, 0), just to throw a little confusion in there fer ya. So if x's are the same
/// and the second y is bigger, then the second coordinate is BELOW the first.
pub fn direc(from: &(usize, usize), to: &(usize, usize)) -> f32 {
    let x1 = from.0 as f32;
    let y1 = from.1 as f32;
    let x2 = to.0 as f32;
    let y2 = to.1 as f32;

    if y2 < y1 && y2 + y1 > x2 + x1 {
        return 0.25;
    } else if x2 > x1 && x2 + x1 > y2 + y1 {
        return 0.5;
    } else if y2 > y1 && y2 + y1 > x2 + x1 {
        return 0.75;
    } else if x2 < x1 && x2 + x1 > y2 + y1 {
        return 1.0;
    }

    if x2 == x1 {
        if y2 > y1 {
            // straight down
            return 0.75;
        } else if y2 < y1 {
            // straight up
            return 0.25;
        }
    }

    if y2 == y1 {
        if x2 > x1 {
            // straight right
            return 0.5;
        } else if x2 < x1 {
            // straight left
            return 1.0;
        }
    }

    // Otherwise it's the same point
    0.0
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(direc(&l1, &l2), 0.0);

        let l1 = (5, 5);
        let l2 = (5, 5);
        assert_eq!(direc(&l1, &l2), 0.0);

        // Generally north
        let l1 = (0, 1);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), 0.25);

        let l1 = (1, 3);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), 0.25);

        // Generally east
        let l1 = (0, 0);
        let l2 = (1, 0);
        assert_eq!(direc(&l1, &l2), 0.50);

        let l1 = (0, 1);
        let l2 = (5, 2);
        assert_eq!(direc(&l1, &l2), 0.50);

        // Generally south
        let l1 = (0, 0);
        let l2 = (0, 1);
        assert_eq!(direc(&l1, &l2), 0.75);

        let l1 = (0, 0);
        let l2 = (1, 2);
        assert_eq!(direc(&l1, &l2), 0.75);

        // Generally west
        let l1 = (1, 0);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), 1.00);

        let l1 = (3, 2);
        let l2 = (0, 0);
        assert_eq!(direc(&l1, &l2), 1.00);
    }

    #[test]
    fn test_randomize() {
        let mut loc = (5, 5);
        randomize(10, &mut loc);
        assert_ne!(loc, (5, 5));
    }
}
