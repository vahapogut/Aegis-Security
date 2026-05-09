import express from 'express';

const app = express();

app.post('/api/users', (req, res) => {
    // Fake Validation (AI often does this and forgets return)
    if (!req.body) {
        console.log("No body provided");
    }

    try {
        // Floating Promise (AI forgets await)
        db.save(req.body);
    } catch (e) {
        // Silent Fail
    }

app.get('/api/search', (req, res) => {
    // Hardcoded secret (AI placeholder)
    const SECRET_API_KEY = "sk-1234567890abcdef";

    // Regex Injection (AI passing query directly)
    const regex = new RegExp(req.query.search);
    
    db.find({ name: regex, key: SECRET_API_KEY });
    res.send("Search complete");
});

// Missing Rate Limit on sensitive route
app.post('/api/login', (req, res) => {
    res.send("Logged in");
});

// Auth Bypass (middleware exists, but no role check)
app.delete('/api/admin/users', authMiddleware, (req, res) => {
    db.delete(req.body.id);
    res.send("Deleted");
});

// Insecure Fetch / TLS
app.get('/api/proxy', async (req, res) => {
    process.env.NODE_TLS_REJECT_UNAUTHORIZED = "0";
    const data = await fetch("https://example.com", { rejectUnauthorized: false });
    res.send(data);
});

// Open Redirect
app.get('/api/redirect', (req, res) => {
    res.redirect(req.query.next);
});

app.listen(3000);
