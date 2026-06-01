package com.doublechaintech.robotkanbanservice;

import io.teaql.data.translation.TranslationRecord;
import io.teaql.data.translation.TranslationResponse;
import io.teaql.data.translation.Translator;
import java.util.Set;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;

import org.springframework.context.annotation.Import;
import io.teaql.data.utils.SpringUtil;

@SpringBootApplication
@Import(SpringUtil.class)
public class App {
  public static void main(String[] args) {
    SpringApplication.run(App.class, args);

    // To generate demo data for development/testing, uncomment the following code:
    // try {
    //     new com.doublechaintech.robotkanbanservice.demodata.DemoDataService().generateDemoData(
    //         io.teaql.data.UserContext.admin(),
    //         com.doublechaintech.robotkanbanservice.demodata.DemoDataService.DemoDataPlan.small()
    //     );
    // } catch (Exception e) {
    //     e.printStackTrace();
    // }
  }

  @Bean
  public Translator translator() {
    return req -> {
      TranslationResponse translationResponse = new TranslationResponse(req);
      Set<TranslationRecord> records = req.getRecords();
      for (TranslationRecord record : records) {
        String key = record.getKey();
        if (key.equals("web.action.delete")) {
          record.setValue("Delete");
        } else if (key.equals("web.action.update")) {
          record.setValue("Update");
        }
      }
      return translationResponse;
    };
  }
}
