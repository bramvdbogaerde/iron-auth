Iron Authentication Library
================================

This repository contains code for an authentication library for the Iron web framework in Rust.
It currently is a work in progress, and not yet in a functioning state.

## Design 

The design of the library is based on how authentication is handled in the Spring Security framework.

The idea is to pass an `AuthenticationContext` along with each request. This context can be changed by 
subsequent filters or middlewares in order to authenticate the current request.

The library will populate the authentication context by asking a provider for `UserDetails`. These `UserDetails` must
include a username and password, and can also include some other information important to the web application
at hand.

## Status

The project is a work in progress, the basic design philosophy and architecture has been defined above, however no integration with 
Iron has been implemented yet.
