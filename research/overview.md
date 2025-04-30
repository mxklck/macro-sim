# Table of Contents
- [Table of Contents](#table-of-contents)
- [Introduction](#introduction)
  - [Initial Scope (Specs \& Requirements)](#initial-scope-specs--requirements)
  - [Agent features](#agent-features)
  - [System dynamics / evolution](#system-dynamics--evolution)
  - [Possible Extensions](#possible-extensions)
  - [Data science based evidence to support assumptions?](#data-science-based-evidence-to-support-assumptions)
- [Plan](#plan)


# Introduction

The goal of this project is to write an agent based macro-economic simulation to investigate how different systems of taxation will impact account distributions of wealth (property) and income within a society. The hope is that we can help discover better systems optimized for the prosperity of all by providing a playground in which these variables can be varied & investigated in a quantitative way.

## Initial Scope (Specs & Requirements)

The initial version of the simulation will contain the following agents:
- States (i.e. countries)
- Companies
- Individuals

We will refer to the system containing all of these agents and their interactions as our *universe*. An evolution algorithm to update the wealth of these agents will be needed. Additionally, the behavior and goals of the agents must be defined. We also need to define some key categories of goods which the agents must / want to consume (essential, luxury etc).

## Agent features
- Agents will forecast and make decisions based on their forecasts. The rough sketch for this behavior is:
  1. Agent looks 3 time steps into the future & tries to pick the action which will enrich it the most
  2. It will attempt to execute this action, but success is probabilistic. The chance of success depends on:
      - The skill of the agent (defined how)
      - The network of the agent
  3. If successful will change behavior, otherwise will maintain status quo
- Consumption will (probably) be linear (evidence to back this up?) in wealth
- Do they have a health property (associated to wealth)?
- Can agents die?
- Can individuals reproduce (inheritance / capital gains tax)?

## System dynamics / evolution

We update the system with discrete timesteps (e.g. months)? In each of these there will be cashflows. Debt is allowed (risk free rate?). 

## Possible Extensions
- Increase the universe hierarchy to allow for multiple states
  - Make one of these states a tax haven to see impact on system
- Unstable governments that change path all the time (the role of stability of direction in facilitating growth)

## Data science based evidence to support assumptions?

Someone should do a literature review...

# Plan

1. Define a simple agent (a person)?
2. Define a simple evolution (i.e. cashflow)?
3. Define a simple forecasting / decision making behavior