package com.ecommerce.freemerkat.entities;

import jakarta.persistence.*;
import jakarta.validation.constraints.NotNull;
import lombok.*;

import java.math.BigDecimal;

@Entity(name = "Product")
@Table(name = "Product")
@Builder
@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class Product {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private Integer id;

    @NotNull
    @Column(unique = true)
    private String name;

    private String description;

    @NotNull
    @Column(precision = 10, scale = 2)
    private BigDecimal price;

    @Column(name = "imgurl")
    private String imgUrl;
}
