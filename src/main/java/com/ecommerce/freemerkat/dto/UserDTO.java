package com.ecommerce.freemerkat.dto;

import com.ecommerce.freemerkat.entities.Order;

import java.util.List;

public record UserDTO(
    String name,
    String email,
    String phone,
    String password,
    List<Order> orders
) { }
