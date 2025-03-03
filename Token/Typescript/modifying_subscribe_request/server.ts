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

  const addresses = address.split(',');

  const validAddresses = addresses.filter(addr => typeof addr === "string" && addr.trim() !== "");

  if (validAddresses.length === 0) {
    return res.status(400).json({ error: "No valid addresses provided" });
  }

  // Add the new addresses to the subscribed list
  subscribedWallets = [...validAddresses];
  res.json({ success: true, wallets: subscribedWallets });
});


app.delete("/wallets/:address", (req, res) => {
  const { address } = req.params;


  if (!subscribedWallets.includes(address)) {
    return res.status(404).json({ error: "Wallet not found" });
  }

  
  subscribedWallets = subscribedWallets.filter((w) => w !== address);
  res.json({ success: true, wallets: subscribedWallets });
});

app.listen(3000, () => console.log("API running on http://localhost:3000"));
