# Scripts de Utilidades

Este directorio contiene scripts útiles para la configuración y manejo de claves.

## Generar Admin Secret Key

### Opción 1: Usando Node.js

1. Instala las dependencias:
```bash
npm install @stellar/stellar-sdk
```

2. Ejecuta el script:
```bash
node generate_keypair.js
```

### Opción 2: Usando Python

1. Instala las dependencias:
```bash
pip install stellar-sdk
```

2. Ejecuta el script:
```bash
python generate_keypair.py
```

### Opción 3: Usando Stellar CLI (si está instalado)

```bash
# Generar nuevo keypair
stellar keys generate admin --network testnet

# Ver la clave generada
stellar keys show admin
```

## Usar las claves generadas

1. **Secret Key (Hex)**: Úsala en tu archivo `.env` como `ADMIN_SECRET_KEY`
2. **Public Key (Address)**: Úsala para inicializar el contrato
3. **Financiar la cuenta**: Ve a https://laboratory.stellar.org/#account-creator?network=test

## Ejemplo de configuración

Si el script genera:
```
Public Key (Address): GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Secret Key (Hex): abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
```

Tu archivo `.env` debería tener:
```env
ADMIN_SECRET_KEY=abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
```

Y al inicializar el contrato usarías:
```bash
curl -X POST http://localhost:3000/init \
  -H "Content-Type: application/json" \
  -d '{"admin_address": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"}'
```