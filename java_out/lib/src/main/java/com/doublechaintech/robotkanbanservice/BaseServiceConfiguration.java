package com.doublechaintech.robotkanbanservice;

import cn.hutool.json.JSONUtil;
import cn.hutool.log.StaticLog;
import io.teaql.data.BaseService;
import io.teaql.data.TQLContext;
import io.teaql.data.UserContext;
import io.teaql.data.checker.CheckException;
import io.teaql.data.web.WebResponse;
import java.util.Map;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.ResponseBody;

@Configuration
public class BaseServiceConfiguration{

    @Bean("robotKanbanServiceBaseService")
    @ConditionalOnMissingBean(name = "robotKanbanServiceBaseService")
    public BaseService baseService(){
        return new BaseService(){};
    }

    @Controller
    public static class BaseController{

        @Autowired
        @Qualifier("robotKanbanServiceBaseService")
        private BaseService baseService;

        @RequestMapping(
               value = "/robotKanbanServiceBaseService/{action}/",
               method = {RequestMethod.POST, RequestMethod.PUT})
        @ResponseBody
        public WebResponse execute(
                @TQLContext UserContext ctx, @PathVariable("action") String action, @RequestBody String parameters) {
            return baseService.execute(ctx, "robotKanbanServiceBaseService", action, parameters);
        }

        @RequestMapping(
                value = "/robotKanbanService/graphql",
                method = {RequestMethod.POST, RequestMethod.PUT})
        @ResponseBody
        public Object graphql(@TQLContext UserContext ctx, @RequestBody String query) {
            Map<String, String> bean = JSONUtil.toBean(query, Map.class);
            return ctx.graphql(bean.get("query"));
        }


        @ExceptionHandler
        @ResponseBody
        public Object handleException(Exception e) {
            if (!(e instanceof CheckException)) {
                StaticLog.error(e);
            }
            return WebResponse.fail(e.getMessage());
        }
    }
}