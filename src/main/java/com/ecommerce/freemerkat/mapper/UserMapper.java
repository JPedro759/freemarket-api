package com.ecommerce.freemerkat.mapper;

import com.ecommerce.freemerkat.dto.UserDTO;
import com.ecommerce.freemerkat.entities.User;
import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;

@Mapper(componentModel = "spring")
public interface UserMapper {
    UserMapper INSTANCE = Mappers.getMapper(UserMapper.class);

    UserDTO toDTO(User user);
    User toEntity(UserDTO dto);
}
