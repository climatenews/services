# Overview

A GraphQL API for databse lookups on transaction data.
Any external requests will be made on the client e.g web3 balance lookups to verify database values.

## Example Queries

```gql

{
  users {
    ...TransactionJoinQuery
    friends {
      ...TransactionJoinQuery
    }
  }
}
    
fragment TransactionJoinQuery on Transaction {
  id
  kind
  name
}
```


```sql
SELECT
      t.transaction_type,
      t.hash,
      t.block_number,
      t.block_timestamp,
      t.from_address,
      t.to_address,
      t.value,
      t.input_method_id,
      s.text_signature as "text_signature?",
      c.name as "contract_name?",
      c.project_id as "project_id?",
      p.name as "project_name?"
    FROM
      transactions as t
      LEFT JOIN signature_info as s ON t.input_method_id = s.hex_signature
      LEFT JOIN contract_info as c ON t.to_address = c.address
      LEFT JOIN project_info as p ON c.project_id = p.project_id
    WHERE
      t.transaction_type = 2
      AND (
        t.from_address = '0x2d80e41e7632bcc30086ebfe2cbdd42710c6f954' OR t.to_address = '0x2d80e41e7632bcc30086ebfe2cbdd42710c6f954'
      )
    ORDER BY
      t.block_number
    LIMIT
      20
      
      
      
      

```