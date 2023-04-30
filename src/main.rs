struct Moeda {
    id: u32,
    nome: String,
    pais: String,
}

struct CrudMoeda {
    moedas: Vec<Moeda>,
}

impl CrudMoeda {
    fn novo() -> Self {
        CrudMoeda {
            moedas: vec![],
        }
    }

    fn adicionar(&mut self, nome: String, pais: String) {
        let id = self.proximo_id();
        let moeda = Moeda { id, nome, pais };
        self.moedas.push(moeda);
    }

    fn listar(&self) {
        for moeda in &self.moedas {
            println!("ID: {}, Nome: {}, País: {}", moeda.id, moeda.nome, moeda.pais);
        }
    }

    fn atualizar(&mut self, id: u32, nome: String, pais: String) {
        for moeda in &mut self.moedas {
            if moeda.id == id {
                moeda.nome = nome;
                moeda.pais = pais;
                break;
            }
        }
    }

    fn remover(&mut self, id: u32) {
        self.moedas.retain(|moeda| moeda.id != id);
    }

    fn proximo_id(&self) -> u32 {
        self.moedas.last().map_or(0, |moeda| moeda.id) + 1
    }
}

fn main() {
    let mut crud_moeda = CrudMoeda::novo();

    println!("=== Principais Moedas ===");

    crud_moeda.adicionar("Real".to_string(), "Brasil".to_string());
    crud_moeda.adicionar("Dólar".to_string(), "Estados Unidos".to_string());
    crud_moeda.adicionar("Euro".to_string(), "União Europeia".to_string());

    println!("Lista de Moedas:");
    crud_moeda.listar();

    crud_moeda.atualizar(1, "Novo Real".to_string(), "Brasil".to_string());
    println!("Lista de Moedas após Atualização:");
    crud_moeda.listar();

    crud_moeda.remover(2);
    println!("Lista de Moedas após Remoção:");
    crud_moeda.listar();
}