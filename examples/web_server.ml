// examples/web_server.ml
import "http"
import "json"
import "collections"

// Estrutura para armazenar dados de usuário
type User = {
    id: int,
    name: string,
    email: string
}

// Banco de dados em memória
let users = Dict<int, User>::new()
let next_id = 1

// Inicializa com alguns usuários
users.insert(1, { id: 1, name: "João", email: "joao@exemplo.com" })
users.insert(2, { id: 2, name: "Maria", email: "maria@exemplo.com" })

// Configura o servidor HTTP
@http_server(port=8080, threads=4)
func handle_request(req: http.Request) -> http.Response {
    match (req.method, req.path) {
        // Rota GET /users - Lista todos os usuários
        ("GET", "/users") => {
            let user_list = users.values()
                .map(|u| json::encode(u))
                .collect::<List<string>>()
            
            http.Response {
                status: 200,
                headers: [("Content-Type", "application/json")],
                body: json::encode(user_list)
            }
        }
        
        // Rota GET /users/:id - Busca usuário específico
        ("GET", path) if path.starts_with("/users/") => {
            let id_str = path.split("/").last()
            let id = id_str.to_int()?
            
            match users.get(id) {
                Some(user) => http.Response {
                    status: 200,
                    headers: [("Content-Type", "application/json")],
                    body: json::encode(user)
                },
                None => http.Response.not_found()
            }
        }
        
        // Rota POST /users - Cria novo usuário
        ("POST", "/users") => {
            let user_data = json::decode<User>(req.body)?
            let id = atomic_add(&next_id, 1)
            
            let new_user = User {
                id: id,
                name: user_data.name,
                email: user_data.email
            }
            
            users.insert(id, new_user)
            
            http.Response {
                status: 201,
                headers: [("Content-Type", "application/json")],
                body: json::encode(new_user)
            }
        }
        
        // Rota PUT /users/:id - Atualiza usuário
        ("PUT", path) if path.starts_with("/users/") => {
            let id = path.split("/").last().to_int()?
            let user_data = json::decode<User>(req.body)?
            
            match users.get(id) {
                Some(_) => {
                    let updated_user = User {
                        id: id,
                        name: user_data.name,
                        email: user_data.email
                    }
                    
                    users.insert(id, updated_user)
                    
                    http.Response {
                        status: 200,
                        headers: [("Content-Type", "application/json")],
                        body: json::encode(updated_user)
                    }
                },
                None => http.Response.not_found()
            }
        }
        
        // Rota DELETE /users/:id - Remove usuário
        ("DELETE", path) if path.starts_with("/users/") => {
            let id = path.split("/").last().to_int()?
            
            match users.remove(id) {
                Some(_) => http.Response.no_content(),
                None => http.Response.not_found()
            }
        }
        
        // Rota padrão
        _ => http.Response.not_found()
    }
}

func main() {
    print("Servidor iniciado em http://localhost:8080")
    print("Rotas disponíveis:")
    print("  GET    /users")
    print("  GET    /users/:id")
    print("  POST   /users")
    print("  PUT    /users/:id")
    print("  DELETE /users/:id")
}