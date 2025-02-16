import express from "express";

const app = express();
app.use(express.json());

let subscribedWallets: string[] = []; // Store wallets in memory

// Get all subscribed wallets
app.get("/wallets", (req, res) => {
  res.json(subscribedWallets);
});

// Add a new wallet
app.get("/wallets/add", (req, res) => {
  const { address } = req.query;

  if (!address || typeof address !== "string") {
    return res.status(400).json({ error: "Address is required and must be a string" });
  }

  // Check if the wallet is already subscribed
  if (subscribedWallets.includes(address)) {
    return res.status(400).json({ error: "Wallet is already subscribed" });
  }

  // Add the wallet to the list
  subscribedWallets.push(address);
  res.json({ success: true, wallets: subscribedWallets });
});

// Remove a wallet
app.delete("/wallets/:address", (req, res) => {
  const { address } = req.params;

  // Check if the wallet exists
  if (!subscribedWallets.includes(address)) {
    return res.status(404).json({ error: "Wallet not found" });
  }

  // Remove the wallet from the list
  subscribedWallets = subscribedWallets.filter((w) => w !== address);
  res.json({ success: true, wallets: subscribedWallets });
});

app.listen(3000, () => console.log("API running on http://localhost:3000"));
