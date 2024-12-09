use aeronet::{io::{connection::{DisconnectReason, Disconnected}, Session, SessionEndpoint}, transport::{lane::LaneIndex, Transport}};
use bevy::{prelude::*, utils::Instant};

pub fn on_connecting(
    trigger: Trigger<OnAdd, SessionEndpoint>,
    names: Query<&Name>,
) {
    let entity = trigger.entity();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");

    info!("{name} connecting");
}

use isent_it::{joueur::{composants::{JoueurPrincipal, Velocity}, spawn_player, animation::AnimationTimer}, network::{Player, TRANSPORT_LANES}};
use rand::random;
pub fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    names: Query<&Name>,
    mut commands: Commands,
    sessions: Query<&Session>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let entity = trigger.entity();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");

    let session = sessions.get(entity)
        .expect("should be connected");

    let mut transport = Transport::new(
        session, 
        TRANSPORT_LANES, 
        TRANSPORT_LANES, 
        Instant::now()
    ).unwrap();

    let player = Player {
        position: Vec3::new(24., 24., 2.),
        skin: random::<u8>()
    };

    let joueur_principal = spawn_player(
        &mut commands, 
        &asset_server, 
        &mut texture_atlas_layouts, 
        player.skin, 
        player.position
    );

    commands.entity(joueur_principal).insert((
        JoueurPrincipal,
        Velocity::default(),
        AnimationTimer(
            Timer::from_seconds(0.1, TimerMode::Repeating)
        )
    ));

    transport.send.push(
        LaneIndex(0), 
        bincode::serialize(&player).unwrap().into(), 
        Instant::now()
    );

    commands.entity(entity).insert(transport);

    info!("{name} connected");
}

pub fn on_disconnected(
    trigger: Trigger<Disconnected>,
    names: Query<&Name>,
) {
    let entity = trigger.entity();
    let Disconnected { reason } = trigger.event();
    let name = names
        .get(entity)
        .expect("our session entity should have a name");
    match reason {
        DisconnectReason::User(reason) => {
            info!("{name} disconnected by user: {reason}");
        }
        DisconnectReason::Peer(reason) => {
            info!("{name} disconnected by peer: {reason}");
        }
        DisconnectReason::Error(err) => {
            error!("{name} disconnected due to error: {err:#}");
        }
    };
}