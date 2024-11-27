use crate::debug::is_debug;
use bevy::prelude::*;

pub struct PluginCollisions;
impl Plugin for PluginCollisions {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        // Outils de debugage
        app.add_systems(Update, draw_collisions.run_if(is_debug));
    }
}
// Collisions

// Le système de collision vérifira
// que le joueur est bien dans l'un des polygones (CollisionArea)
#[derive(Component)]
pub struct CollisionArea(Vec<Vec2>);

#[derive(Component)]
pub struct CollisionDisabled;

fn setup(mut commands: Commands) {
    // Le polygone du premier étage
    commands.spawn(CollisionArea(Vec::from([
        Vec2::new(-20., 9.),
        Vec2::new(452., 9.),
        Vec2::new(476., 33.),
        Vec2::new(4., 33.),
    ])));
}

// Ce système de débuggage dessine les bordures des zones de collisions
fn draw_collisions(
    collisions: Query<&CollisionArea, Without<CollisionDisabled>>,
    mut draw: Gizmos,
) {
    // En rouge
    let c = Color::linear_rgb(255.0, 0.0, 0.0);

    // Chaque zones de collisions
    for area in collisions.iter() {
        // Nombre de sommets
        let n = area.0.len();
        for i in 0..n {
            // On dessine un trait entre les sommets adjacents
            draw.line_2d(area.0[i], area.0[(i + 1) % n], c);
        }
    }
}

// Fonction qui vérifie qu'un point est bien dans un polygone
// Code généré par Github Copilot
pub fn point_in_area(point: Vec2, polygon: &CollisionArea) -> bool {
    let mut is_inside = false;
    let mut j = polygon.0.len() - 1;
    for i in 0..polygon.0.len() {
        let pi = polygon.0[i];
        let pj = polygon.0[j];
        if (pi.y > point.y) != (pj.y > point.y)
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            is_inside = !is_inside;
        }
        j = i;
    }
    is_inside
}

// Tests pour vérifier que la fonction ci-dessus fonctionne correctement
// On peut lancer les tests via un bouton ci-contre avec VSCode ou
// avec la commande `cargo test`
#[cfg(test)]
mod tests {
    use crate::collisions::{point_in_area, CollisionArea};
    use bevy::prelude::*;

    #[test]
    fn point_in_rect() {
        let polygon = CollisionArea(Vec::from([
            Vec2::new(0., 0.),
            Vec2::new(480., 0.),
            Vec2::new(0., 48.),
            Vec2::new(480., 48.),
        ]));
        let points_hors_polygon = Vec::from([
            Vec2::new(40., -20.),  // En dessous
            Vec2::new(40., 49.),   // Au dessus
            Vec2::new(-20., 20.),  // A gauche
            Vec2::new(490., 20.),  // A droite
            Vec2::new(490., 49.),  // En dessus à droite
            Vec2::new(-20., 49.),  // En dessus à gauche
            Vec2::new(490., -20.), // Au dessous à droite
            Vec2::new(-20., -20.), // Au dessous à gauche
        ]);
        let points_du_polygon = Vec2::new(229., 38.);

        for point in points_hors_polygon {
            assert_eq!(false, point_in_area(point, &polygon));
        }
        println!("Points hors du polygone ✅");

        assert_eq!(true, point_in_area(points_du_polygon, &polygon));
        println!("Points du polygone ✅");
    }

    #[test]
    fn point_in_parallelogramme() {
        let polygon = CollisionArea(Vec::from([
            Vec2::new(0., 0.),
            Vec2::new(480., 0.),
            Vec2::new(504., 24.),
            Vec2::new(24., 24.),
        ]));
        let points_hors_polygon = Vec::from([
            Vec2::new(12., 36.),
            Vec2::new(500., 36.),
            Vec2::new(500., 5.),
        ]);
        let points_du_polygon = Vec::from([
            Vec2::new(229., 12.),
            Vec2::new(36., 12.),
            Vec2::new(502., 23.),
        ]);

        for point in points_hors_polygon {
            assert_eq!(false, point_in_area(point, &polygon));
        }
        println!("Points hors du polygone ✅");

        for point in points_du_polygon {
            assert_eq!(
                true,
                point_in_area(point, &polygon),
                "points: (x: {}, y: {})",
                point.x,
                point.y
            );
        }
        println!("Points du polygone ✅");
    }
}
