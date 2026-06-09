package com.doublechaintech.robotkanbanservice;

import cn.hutool.log.StaticLog;
import io.teaql.data.checker.CheckException;
import org.springframework.context.annotation.Configuration;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseBody;

@Configuration
public class ServiceRequestConfiguration {

    @Controller
    public static class ServiceRequestController{
         @ExceptionHandler
         @ResponseBody
         public Object handleException(Exception e) {
             if (!(e instanceof CheckException)) {
                 StaticLog.error(e);
             }
             return e;
         }
    }
}