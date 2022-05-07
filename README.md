# JGres

## Descrição
O JGres consiste numa extensão que permite a criação e manipulação de tabelas utilizando a notação `JSON` (um exemplo pode ser encontrado nesse [arquivo](https://github.com/hizagi/jgres/blob/main/test.json)), o caminho do arquivo json é passado como parâmetro da função `run_json` que traduz e executa o SQL gerado, a função `plan_json` permite apenas traduzir o arquivo e retorna o SQL gerado no próprio console do banco, sem executar o SQL gerado, ainda temos a função `plan_json_with_output` que faz o mesmo que a função `plan_json` enviando o SQL para o arquivo especificado nos parâmetros (exemplo: [arquivo](https://github.com/hizagi/jgres/blob/main/output.sql)).

### Exemplos:

- `run_json`
```
SELECT run_json("path/to/json/file");
```

- `plan_json`
```
SELECT plan_json("path/to/json/file");
```

- `plan_json_with_output`
```
SELECT plan_json_with_output("path/to/json/file", "path/to/output/sql/file");
```