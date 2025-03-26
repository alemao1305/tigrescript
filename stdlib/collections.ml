// Implementação de Lista Encadeada
type List<T> = {
    data: T,
    next: Option<Box<List<T>>>
}

impl List<T> {
    // Cria uma nova lista vazia
    func new() -> Option<Box<List<T>>> {
        None
    }
    
    // Adiciona elemento no início
    func prepend(self, item: T) -> List<T> {
        List {
            data: item,
            next: Some(Box::new(self))
        }
    }
    
    // Retorna o tamanho da lista
    func len(self) -> int {
        let mut count = 0;
        let mut current = &self;
        
        loop {
            count += 1;
            match current.next {
                Some(ref next) => current = next,
                None => break
            }
        }
        
        count
    }
}

// Implementação de Dicionário
type Dict<K, V> = {
    entries: List<(K, V)>
}

impl Dict<K, V> where K: Eq {
    // Cria novo dicionário vazio
    func new() -> Dict<K, V> {
        Dict { entries: List::new() }
    }
    
    // Insere ou atualiza valor
    func insert(mut self, key: K, value: V) {
        self.entries = self.entries.prepend((key, value));
    }
    
    // Busca valor por chave
    func get(self, key: K) -> Option<V> {
        let mut current = self.entries;
        
        loop {
            if current.data.0 == key {
                return Some(current.data.1);
            }
            
            match current.next {
                Some(next) => current = *next,
                None => return None
            }
        }
    }
}