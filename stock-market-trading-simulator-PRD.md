# Product Requirements Document (PRD) for Stock Market Trading Simulator

## 1. Overview

### 1.1 Purpose
The Stock Market Trading Simulator is a web-based application that allows users to practice trading stocks with virtual money in a simulated market environment. The platform provides real-time simulated stock prices, enables execution of buy/sell orders, manages user portfolios, and maintains a transaction history. This project will showcase full-stack development skills, cloud deployment, performance logging, and testing best practices.

### 1.2 Scope
- **Simulated Stock Data:** Generate real-time stock prices using a simulation algorithm.
- **Trading Engine:** Process buy and sell orders, update user portfolios, and record transactions.
- **Portfolio Management:** Display virtual cash balance, stock holdings, and trading history.
- **User Interface:** A responsive web dashboard using NextJS with TypeScript.
- **Backend API:** RESTful API built with FastAPI, Python, and MySQL.
- **Logging & Performance:** Detailed logging with performance metrics extracted from Azure logs.
- **Testing & Deployment:** Automated tests with Pytest and Postman; deployment on Azure (VM or AppEngine) with optional Docker containerization.

### 1.3 Audience
- Recruiters and hiring managers evaluating full-stack development skills.
- Developers building portfolio projects to demonstrate expertise in web applications and cloud deployments.

## 2. Objectives & Goals
- **Simulate a stock market:** Use a random walk or similar algorithm to update stock prices.
- **Enable trading operations:** Allow users to execute buy and sell orders.
- **Track portfolios:** Display cash balance, stock holdings, and transaction logs.
- **Monitor performance:** Integrate logging to track the time spent in order processing and API routing.
- **Ensure quality:** Build comprehensive tests using Pytest and validate endpoints with Postman.
- **Deploy to the cloud:** Host the application on Azure and extract logs for performance analysis.

## 3. User Stories
- **Stock Viewer:** *As a user, I want to view current simulated stock prices so that I can decide which stocks to trade.*
- **Trader:** *As a user, I want to place buy and sell orders to manage my virtual portfolio.*
- **Portfolio Manager:** *As a user, I want to see my portfolio details, including cash and stock holdings, so that I can track my trading performance.*
- **Performance Analyst:** *As a developer, I want to extract and analyze logs from the Azure deployment to understand the performance of API operations.*
- **Authenticated User (Optional):** *As a user, I want to register and log in securely so that my data is protected.*

## 4. Features & Requirements

### 4.1 Stock Data Simulation
- **Endpoints:**
  - `GET /stocks`: Retrieve a list of stocks with current simulated prices.
  - `GET /stocks/{symbol}`: Retrieve detailed historical data for a specific stock.
- **Requirements:**
  - Use a random walk algorithm or predefined dataset to simulate price changes.
  - Update prices periodically via a background task or scheduler.

### 4.2 Trading Engine
- **Endpoints:**
  - `POST /trade/buy`: Place a buy order. Request should include user ID, stock symbol, and quantity.
  - `POST /trade/sell`: Place a sell order with similar parameters.
- **Requirements:**
  - Validate orders against the user's cash balance or stock holdings.
  - Update the portfolio and record transactions upon successful trades.

### 4.3 Portfolio Management
- **Endpoint:**
  - `GET /portfolio`: Retrieve the user’s cash balance, stock holdings, and transaction history.
- **Requirements:**
  - Maintain accurate transaction logs.
  - Provide real-time updates of the portfolio status.

### 4.4 User Authentication (Optional)
- **Endpoints:**
  - `POST /auth/register`: Register a new user.
  - `POST /auth/login`: Authenticate a user and issue a JWT token.
- **Requirements:**
  - Secure password storage (e.g., using bcrypt).
  - Implement token-based authentication for secure session management.

### 4.5 Logging & Performance Monitoring
- **Requirements:**
  - Integrate structured logging in FastAPI to capture key operations (e.g., order processing, API routing).
  - **Azure Task:** Extract logs from the Azure VM or AppEngine to generate a summary report detailing time spent in order processing, database operations, and API responses.
  - Example log note: “Played with logs to identify time spent in order processing across API layers.”

### 4.6 Testing & Quality Assurance
- **Requirements:**
  - Write unit and integration tests using Pytest.
  - Develop a comprehensive Postman collection for manual endpoint testing.
  - Integrate testing into a CI/CD pipeline for automated verification.

### 4.7 Deployment
- **Requirements:**
  - Deploy the backend on Azure (VM or AppEngine).
  - Optionally, create a Dockerfile to containerize the application.
  - Configure environment variables for database connections and secret keys.
  - Set up CI/CD for automated deployment and testing.

## 5. Non-Functional Requirements
- **Performance:** Ensure low-latency API responses and efficient order processing.
- **Scalability:** The architecture should support increased load with minimal changes.
- **Security:** Implement secure authentication and protect against vulnerabilities such as SQL injection and XSS.
- **Maintainability:** Write clear, modular code with thorough documentation and tests.

## 6. Milestones & Timeline
- **Hours 1–15:** Requirements definition, database schema design, and project setup.
- **Hours 16–40:** Develop stock simulation endpoints and trading engine (buy/sell order processing).
- **Hours 41–60:** Implement portfolio management and logging functionality.
- **Hours 61–75:** Build the frontend dashboard using NextJS and integrate API calls.
- **Hours 76–100:** Deploy on Azure, extract/analyze logs, complete testing (Pytest & Postman), and polish UI/UX.

## 7. Success Metrics
- **Functionality:** Successful execution of simulated trades and real-time portfolio updates.
- **Testing:** High test coverage across API endpoints and smooth CI/CD integration.
- **Performance:** Detailed log reports showcasing efficient API operations.
- **User Experience:** A responsive and intuitive web interface built with NextJS.
