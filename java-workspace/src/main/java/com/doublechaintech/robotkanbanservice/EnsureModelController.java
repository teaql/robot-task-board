package com.doublechaintech.robotkanbanservice;

import cn.hutool.core.map.MapUtil;
import io.teaql.data.TQLContext;
import io.teaql.data.UserContext;
import io.teaql.data.meta.EntityMetaFactory;
import io.teaql.data.sql.SQLRepositorySchemaHelper;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ResponseBody;

@Controller
public class EnsureModelController {
  @Autowired private EntityMetaFactory factory;

  @GetMapping("/ensureDB")
  @ResponseBody
  public Object ensureTable(@TQLContext UserContext context) {
    try {
      new SQLRepositorySchemaHelper().ensureSchema(context, factory);
      return MapUtil.of("ok", true);
    } catch (Exception e) {
      e.printStackTrace();
      return MapUtil.of("fail", e.getMessage());
    }
  }

  @GetMapping("/version")
  @ResponseBody
  public Object version(@TQLContext UserContext context) {
    return MapUtil.of("version", "1.0.0");
  }
}
