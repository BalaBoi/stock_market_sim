# Engineering Design Document for Stock Market Trading Simulator

## 1. System Architecture

### 1.1 Overview
The Stock Market Trading Simulator is built using a client-server architecture:
- **Frontend:** A responsive web dashboard created with NextJS and TypeScript.
- **Backend:** A RESTful API using FastAPI and Python.
- **Database:** MySQL is used for persistent storage of user data, stock data, transactions, and portfolio information.
- **Deployment:** Hosted on Azure (VM or AppEngine) with optional Docker containerization.
- **Logging & Monitoring:** Integrated structured logging with scripts to extract and analyze logs from the Azure environment.

### 1.2 Architecture Diagram

[Frontend (NextJS)] <--HTTPS--> [Backend API (FastAPI)] | V [MySQL Database] | V [Azure Deployment Environment]


## 2. Technology Stack
- **Frontend:** NextJS, TypeScript, CSS/SCSS
- **Backend:** Python, FastAPI, SQLAlchemy (ORM)
- **Database:** MySQL
- **Testing:** Pytest for unit and integration tests, Postman for API testing
- **Cloud:** Azure (VM or AppEngine)
- **Containerization:** Docker (optional)

## 3. Data Model

### 3.1 Database Schema
- **Users Table**
  - `id`: Primary key, auto-increment.
  - `username`: Unique, string.
  - `email`: Unique, string.
  - `password_hash`: Securely hashed password.
  - `cash_balance`: Decimal, representing the virtual cash available.
- **Stocks Table**
  - `symbol`: Primary key, string (e.g., AAPL, GOOGL).
  - `name`: Stock name.
  - `current_price`: Decimal representing the current simulated price.
- **Transactions Table**
  - `id`: Primary key, auto-increment.
  - `user_id`: Foreign key referencing Users.
  - `stock_symbol`: Foreign key referencing Stocks.
  - `order_type`: Enum (`buy` or `sell`).
  - `quantity`: Integer.
  - `price`: Decimal (price at transaction time).
  - `timestamp`: DateTime.
- **Portfolio Table (Optional)**
  - `user_id`: Foreign key referencing Users.
  - `stock_symbol`: Foreign key referencing Stocks.
  - `quantity`: Integer representing current holdings.

## 4. API Design

### 4.1 Endpoints
- **Stock Data**
  - `GET /stocks`: Returns a list of stocks with current simulated prices.
  - `GET /stocks/{symbol}`: Returns historical price data for the specified stock.
- **Trading**
  - `POST /trade/buy`: Executes a buy order.  
    **Payload:** `{ "user_id": int, "stock_symbol": string, "quantity": int }`
  - `POST /trade/sell`: Executes a sell order.  
    **Payload:** `{ "user_id": int, "stock_symbol": string, "quantity": int }`
- **Portfolio Management**
  - `GET /portfolio`: Returns the user’s cash balance, stock holdings, and transaction history.
- **Authentication (Optional)**
  - `POST /auth/register`: Registers a new user.
  - `POST /auth/login`: Authenticates a user and returns a JWT token.

### 4.2 Data Flow
1. **Stock Price Simulation:**
   - A background task (or scheduler) updates stock prices using a random walk algorithm.
   - Stock data endpoints retrieve the latest prices from the database.
2. **Order Execution:**
   - The user sends a buy/sell order via the API.
   - The API validates the request by checking cash balance (for buys) or current holdings (for sells).
   - The order is processed, updating the portfolio and recording the transaction.
   - A response is returned to the user indicating success or failure.
3. **Logging Flow:**
   - Key events (API request start/end, order validation, transaction processing) are logged.
   - Logs are stored locally or sent to a centralized logging service.
   - A separate script extracts and parses logs from Azure to generate performance metrics.

## 5. Logging & Monitoring
- **Implementation:**
  - Use Python's built-in logging module or a third-party library (e.g., Loguru) to capture structured logs.
  - Log critical operations such as API entry/exit, order processing, database queries, and errors.
- **Azure Log Extraction:**
  - Develop a Python script to extract logs from the Azure environment.
  - Parse logs to create a performance summary, highlighting time spent on order processing, database operations, and API routing.
  - Include statements like “Played with logs to identify time spent in order processing across API layers.”

## 6. Testing Strategy
- **Unit Testing:**
  - Write tests for individual functions and components (e.g., simulation algorithm, trading logic).
- **Integration Testing:**
  - Use Pytest to simulate end-to-end scenarios (placing orders, updating portfolios, etc.).
- **Postman Collection:**
  - Create a collection covering all endpoints for manual testing and regression checks.
- **CI/CD:**
  - Integrate testing into a CI/CD pipeline (e.g., using GitHub Actions or Azure DevOps) for automated testing on each commit.

## 7. Deployment Considerations
- **Azure Deployment:**
  - Deploy the backend on an Azure VM or AppEngine instance.
  - Set up environment variables for database connections, JWT secrets, and other configurations.
- **Containerization:**
  - Optionally, create a Dockerfile to containerize the application for consistent deployment.
- **CI/CD Pipeline:**
  - Automate build, test, and deployment processes to streamline updates and ensure stability.

## 8. Performance Considerations
- **Caching:**  
  - Consider caching frequently requested endpoints (e.g., stock data) to reduce database load.
- **Database Optimization:**  
  - Use indexing and optimized queries for the trading engine and portfolio lookups.
- **Scalability:**  
  - Design the system so that additional instances can be added to handle increased load.

## 9. Milestones & Timeline (Engineering)
- **Phase 1 (1–15 Hours):**
  - Set up the project repository, environment, and database schema.
- **Phase 2 (16–40 Hours):**
  - Develop stock simulation endpoints and implement the trading engine.
  - Integrate logging in key API endpoints.
- **Phase 3 (41–60 Hours):**
  - Build portfolio management and authentication (optional) endpoints.
  - Write initial unit and integration tests.
- **Phase 4 (61–75 Hours):**
  - Develop the NextJS frontend and integrate it with the backend API.
- **Phase 5 (76–100 Hours):**
  - Deploy on Azure, extract and analyze logs, finalize testing, and polish the UI/UX.

## 10. Open Issues & Future Enhancements
- **Enhancements:**
  - Implement real-time updates via WebSockets.
  - Add advanced analytics for user trading performance.
  - Integrate real market data for a hybrid simulation model.
- **Open Issues:**
  - Optimizing concurrent order processing under high load.
  - Fine-tuning the simulation algorithm for realistic stock behavior.