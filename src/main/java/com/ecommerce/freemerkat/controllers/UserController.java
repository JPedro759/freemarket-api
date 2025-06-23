package com.ecommerce.freemerkat.controllers;

import com.ecommerce.freemerkat.entities.User;
import com.ecommerce.freemerkat.repositories.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/users")
public class UserController {

    @Autowired
    private UserRepository userRepository;

    @GetMapping
    public ResponseEntity<List<User>> getUsers() {
        List<User> listUsers = userRepository.findAll();
        return ResponseEntity.status(HttpStatus.OK).body(listUsers);
    }
}
