# JGres

## Descrição
O JGres consiste numa extensão que permite a criação e manipulação de tabelas utilizando a notação `JSON` (um exemplo pode ser encontrado nesse [arquivo](https://github.com/hizagi/jgres/blob/main/test.json)), o caminho do arquivo json é passado como parametro da função `run_json` que traduz e executa o SQL gerado, a função `plan_json` ainda está em desenvolvimento, que permite apenas traduzir o arquivo, sem executar o SQL gerado.

```
SELECT run_json("path/to/json/file")
```