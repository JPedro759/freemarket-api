package com.ecommerce.freemerkat.mapper;

import com.ecommerce.freemerkat.dto.OrderDTO;
import com.ecommerce.freemerkat.dto.ProductDTO;
import com.ecommerce.freemerkat.entities.Order;
import com.ecommerce.freemerkat.entities.Product;
import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;

@Mapper(componentModel = "spring")
public interface ProductMapper {
    ProductMapper INSTANCE = Mappers.getMapper(ProductMapper.class);

    ProductDTO toDTO(Product order);
    Product toEntity(ProductDTO dto);
}
