use aeronet::{io::{connection::{DisconnectReason, Disconnected}, Session, SessionEndpoint}, transport::{lane::LaneIndex, Transport}};
use bevy::{prelude::*, utils::Instant};

// Lorsque l'utilisateur se connecte
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

use isent_it::{joueur::{composants::{JoueurPrincipal, Velocity}, spawn_player}, network::{Player, TRANSPORT_LANES}};
use rand::random;

// Lorsqu'il parvient à se connecter
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

    // On ajoute le composant Transport pour pouvoir fragmenter 
    // les messages en packet et les reconstruirent.
    // Pour envoyer des données de plusieurs types et de tailles très variées
    let mut transport = Transport::new(
        session, 
        TRANSPORT_LANES, 
        TRANSPORT_LANES, 
        Instant::now()
    ).unwrap();

    // Les données partagés avec le serveur
    let player = Player::new(
        Vec3::new(24., 24., 15.), 
        random::<u8>()
    );

    // L'instance du joueur côté client
    let joueur_principal = spawn_player(
        &mut commands, 
        &asset_server, 
        &mut texture_atlas_layouts, 
        player.clone()
    );

    commands.entity(joueur_principal).insert((
        JoueurPrincipal,
        Velocity::default()
    ));

    // On se présente au serveur
    transport.send.push(
        LaneIndex(0), 
        bincode::serialize(&player).unwrap().into(), 
        Instant::now()
    );

    commands.entity(entity).insert(transport);

    info!("{name} connected");
}

// Lorsqu'on se déconnecte.
pub fn on_disconnected(
    trigger: Trigger<Disconnected>,
    local_players: Query<Entity, With<JoueurPrincipal>>,
    mut commands: Commands,
    names: Query<&Name>,
) {
    for local_player in local_players.iter() {
        commands.entity(local_player).despawn();
    }

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