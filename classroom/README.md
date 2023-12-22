## tailscale
まず下のやつでmagicdnsにlocalhost:8080に割り当てる
```
tailscale serve 8080
```
## classroomコンテナ内
```
prisma generate --schema=/dry/postgresql/schema.sql
prisma migrate dev --schema=/dry/postgresql/schema.sql
```
