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

    async drawEvent() {
        const res = await fetch('/api/draw_event');
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

    // ─── Debug Endpoints ────────────────────────────────
    async getJobs() {
        const res = await fetch('/api/jobs');
        return res.json();
    },

    async debugSkipStage() {
        const res = await fetch('/api/debug/skip_stage', { method: 'POST' });
        return res.json();
    },

    async debugSetStats(stats) {
        const res = await fetch('/api/debug/set_stats', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(stats),
        });
        return res.json();
    },

    async debugGrantTag(tag) {
        const res = await fetch('/api/debug/grant_tag', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tag }),
        });
        return res.json();
    },
};
