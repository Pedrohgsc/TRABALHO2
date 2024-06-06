Definição da Estrutura da Reserva:
Reserva contém os detalhes da reserva: nome do hotel, número do quarto, data de check-in e check-out.

Função Hash Original:
gerar_hash cria um valor hash a partir de uma string alfanumérica. Utiliza a técnica de multiplicação e adição dos bytes da string para gerar o hash. 

Estrutura de Dados
SistemaReservas utiliza um HashMap onde a chave é o hash gerado e o valor é um vetor de Reserva para tratar colisões. 

Operações:
Inserir: inserir_reserva insere uma nova reserva, adicionando ao vetor correspondente no HashMap.
Recuperar: recuperar_reserva retorna as reservas associadas ao código da reserva.
Remover: remover_reserva remove a primeira reserva no vetor correspondente. Se o vetor ficar vazio, a entrada é removida do HashMap.
Este código cria um sistema básico de gerenciamento de reservas de hotel, utilizando um hash customizado e tratamento de colisões por encadeamento.
