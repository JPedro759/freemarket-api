package com.ecommerce.freemerkat.mapper;

import com.ecommerce.freemerkat.dto.OrderDTO;
import com.ecommerce.freemerkat.entities.Order;
import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;

@Mapper(componentModel = "spring")
public interface OrderMapper {
    OrderMapper INSTANCE = Mappers.getMapper(OrderMapper.class);

    OrderDTO toDTO(Order order);
    Order toEntity(OrderDTO dto);
}
