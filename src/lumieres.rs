use bevy::{
    color::palettes::css::WHITE, 
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, 
    prelude::*, 
    render::camera::RenderTarget
};
use bevy_magic_light_2d::prelude::*;

pub struct PluginLumieres;

impl Plugin for PluginLumieres {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(
            Color::srgba_u8(255, 255, 255, 0)
        ));
        app.register_type::<OmniLightSource2D>();
        app.add_plugins(BevyMagicLight2DPlugin);
        app.add_systems(Startup, 
            setup.after(setup_post_processing_camera));
    }
}

fn setup(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    let mut lights = vec![];

    for i in 0..5 {
        let light = commands
            .spawn(Name::new(format!("light-{}", i)))
            .insert(SpatialBundle::from_transform(
                Transform::from_translation(
                    Vec3::new(24. + 96. * i as f32, 72., 5.)
                )
            ))
            .insert(OmniLightSource2D {
                intensity:          1.5,
                falloff:            Vec3::new(50.0, 1.0, 0.05),
                color:              Color::srgb_u8(50, 50, 50),
                ..default()
            }).id();
            lights.push(light);
    }

    commands.spawn((
        SkylightLight2D {
            color:     Color::srgb_u8(100, 100, 100),
            intensity: 0.01,
        },
        Name::new("ambient_light"),
    ));

    let camera = Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(WHITE.into()),
            hdr: true,
            target: RenderTarget::Image(camera_targets.floor_target.clone()),
            ..default()
        },
        transform: Transform::from_translation(
            // On centre l'axe y de la camera sur le premier étage
            Vec3::with_y(Vec3::ZERO, 36.0)
        ),
        tonemapping: Tonemapping::Reinhard,
        ..default()
    };
    
    
    commands.spawn((
        Name::new("main_camera"),
        camera, 
        FloorCamera,
        BloomSettings::OLD_SCHOOL
    ))
        .insert(SpriteCamera);
}