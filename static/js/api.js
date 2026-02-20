// api.js — Fetch wrappers for REST API calls

const API = {
    async newGame(seed = null) {
        const res = await fetch('/api/new_game', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(seed ? { seed } : {}),
        });
        return res.json();
    },

    async getState() {
        const res = await fetch('/api/state');
        return res.json();
    },

    async getPhaseData() {
        const res = await fetch('/api/phase_data');
        return res.json();
    },

    async submitTurn(choices) {
        const res = await fetch('/api/submit_turn', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(choices),
        });
        return res.json();
    },

    async getEnding() {
        const res = await fetch('/api/endings');
        return res.json();
    },

    async health() {
        const res = await fetch('/api/health');
        return res.json();
    },
};
