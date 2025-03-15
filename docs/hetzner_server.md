# Hetzner server

### ports overview

| service | port |
|---------|------|
| api | 3000 |

## Services

### Qdrant

```bash
docker run -d --name qdrant -p 6333:6333 qdrant/qdrant
```

### Neo4j

```bash
docker run -d --name neo4j -p 7474:7474 -p 7687:7687 \
  -e NEO4J_AUTH=neo4j/hardToNeo4j \
  neo4j
```


