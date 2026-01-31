# my-weather-api - next Framework

An x402-enabled API built on aptos blockchain.

## Features

- Payment-enabled API endpoints
- Automated wallet management
- Development facilitator integration

## Getting Started

```bash
# Install dependencies
npm install

# Run the development server
npm run dev

# Start the facilitator
x402 facilitator start
```

## Configuration

See `config/x402.toml` for project configuration.

## Testing Payment Flows

```bash
x402 test payment --api http://localhost:3000/api --amount 1000
```

## License

MIT
