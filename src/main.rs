use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::server::conn::AddrStream;
use std::{convert::Infallible, net::SocketAddr};
use std::net::IpAddr;

use structopt::StructOpt; // Importe le trait StructOpt

/// Options de ligne de commande pour le répartiteur de charge.
#[derive(StructOpt)]
struct CmdOptions {
    #[structopt(help = "URL des serveurs amont")]
    upstream: Vec<String>,
}

/// Gère les requêtes entrantes et les redirige vers le serveur amont approprié.
async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path().starts_with("/first") {
        
        match hyper_reverse_proxy::call(client_ip, "http://arr.homelab.lan/", req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else if req.uri().path().starts_with("/second") {
        
        match hyper_reverse_proxy::call(client_ip, "http://cloud.homelab.lan", req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else {
        // Gestionnaire de requêtes de débogage
        debug_request(req)
    }
}

/// Gestionnaire de requêtes de débogage renvoyant une réponse contenant les détails de la requête.
fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

/// Lance le répartiteur de charge avec les serveurs amont spécifiés.
#[tokio::main]
async fn main() {
    // Analyse des arguments de ligne de commande passés à ce programme
    let options = CmdOptions::from_args();

    if options.upstream.is_empty() {
        eprintln!("Au moins un serveur amont doit être spécifié avec l'option --upstream.");
        std::process::exit(1);
    }

    // Le reste de votre code existant
    let bind_addr = "127.0.0.1:8080";
    let addr: SocketAddr = bind_addr.parse().expect("Impossible de parser l'adresse IP : port.");

    // Crée un service qui génère des instances de la fonction handle pour chaque connexion.
    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| handle(remote_addr, req)))
        }
    });

    // Crée un serveur Hyper lié à l'adresse spécifiée et sert le service make_svc.
    let server = Server::bind(&addr).serve(make_svc);

    println!("Serveur en cours d'exécution sur {:?}", addr);

    // Démarre le serveur et gère les erreurs éventuelles.
    if let Err(e) = server.await {
        eprintln!("Erreur du serveur : {}", e);
    }
}
