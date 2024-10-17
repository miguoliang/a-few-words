module.exports = {
    'a-few-words-api': {
        input: {
            target: 'http://127.0.0.1:8000/api-docs/openapi.json',
        },
        output: {
            target: './src/api.ts',
        },
        hooks: {
            afterAllFilesWrite: 'prettier --write',
        }
    },
};