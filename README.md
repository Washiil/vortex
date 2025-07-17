# Vortex: A High-Performance Order Book & Matching Engine
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)]()

Vortex is a high-performance financial limit order book (LOB) and matching engine written in **Rust**. It's designed for simulating market microstructure dynamics and testing quantitative trading strategies in a low-latency environment.

---

## Core Features

* **Low-Latency Matching Engine:** An in-memory engine capable of processing and matching **limit** and **market** orders with high throughput.
* **Real-Time Data Ingestion:** Includes a WebSocket client for connecting to live L2/L3 market data feeds from major exchanges (e.g., Coinbase).
* **Quantitative Signal Calculation:** Computes key microstructure indicators in real-time as the order book state changes.
* **Strategy Simulation Framework:** A modular interface for implementing and testing automated trading strategies that react to market events and quantitative signals.

---

## Technical Stack & Architecture

Vortex is built with **Rust** for its performance, memory safety, and robust concurrency features.

* **Core Language:** Rust
* **Asynchronous Runtime:** `tokio` for high-performance, non-blocking I/O.
* **Data Structures:** The order book leverages `BTreeMap` for sorted price levels and `VecDeque` for order queues, ensuring efficient insertions, deletions, and lookups.

The system is designed around a **single-threaded matching engine core** to ensure deterministic and sequential processing of orders, preventing race conditions. Asynchronous tasks handle network I/O (like WebSocket data) and communicate with the core engine via message-passing channels (`tokio::mpsc`), a common and effective pattern in high-performance trading systems.

---

## Quantitative Indicators Implemented

A key focus of this project is the analysis of market microstructure. The following indicators are calculated from the live order book data:

* **Micro-Price:** A volume-weighted price that provides a better estimate of the true market price than the standard mid-price.
    $$
    \text{Micro-Price} = \frac{(\text{Best Bid} \times \text{Ask Size}) + (\text{Best Ask} \times \text{Bid Size})}{\text{Bid Size} + \text{Ask Size}}
    $$
* **Order Book Imbalance (OBI):** Measures the ratio of buy-side to sell-side pressure, often used as a short-term predictor of price movements.
    $$
    \text{OBI} = \frac{\text{Total Bid Volume}}{\text{Total Bid Volume} + \text{Total Ask Volume}}
    $$
* **Volume-Weighted Average Price (VWAP):** The average execution price over a time window, weighted by trade volume. Used as a benchmark for execution quality.

---

## Getting Started

### Prerequisites

* Rust toolchain

### Installation & Running

1.  Clone the repository:
    ```sh
    git clone https://github.com/Washiil/vortex.git
    cd vortex
    ```
2.  Build and run the project:
    ```sh
    cargo run --release
    ```

### Running Tests

To ensure correctness and prevent regressions, the project includes a suite of unit and integration tests.

```sh
cargo test