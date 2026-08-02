#!/bin/bash

# ==========================================
# 自动监控并运行脚本 (Auto Run Script)
# ==========================================

API_URL="https://api.teaql.io/latest/version/"
VERSION_FILE=".current_teaql_version"
EMAIL_SCRIPT="./send_email.sh" # 发生错误时调用的邮件脚本

# 1. 从 API 抓取最新版本号
# 通过过滤 markdown 表格中的 teaql-rs，并提取反引号内的版本号
LATEST_VERSION=$(curl -s "$API_URL" | grep "teaql-rs" | awk -F '`' '{print $2}')

if [ -z "$LATEST_VERSION" ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S') [Error] 无法从 API 获取最新版本号"
    exit 1
fi

# 2. 读取本地记录的当前版本
if [ -f "$VERSION_FILE" ]; then
    CURRENT_VERSION=$(cat "$VERSION_FILE")
else
    CURRENT_VERSION="unknown"
fi

# 3. 比对版本号
if [ "$LATEST_VERSION" != "$CURRENT_VERSION" ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S') [Info] 发现新版本: $CURRENT_VERSION -> $LATEST_VERSION. 开始运行..."
    
    # ------------------------------------------------------------------
    # 这里放置您要“立即运行”的命令 (例如自动替换 Cargo.toml 版本并跑测试)
    # ------------------------------------------------------------------
    # 示例: 运行全量测试
    cargo test
    RUN_EXIT_CODE=$?
    
    # 4. 处理运行结果
    if [ $RUN_EXIT_CODE -ne 0 ]; then
        echo "$(date '+%Y-%m-%d %H:%M:%S') [Error] 运行失败！正在触发邮件通知..."
        
        if [ -x "$EMAIL_SCRIPT" ]; then
            # 调用您提供的邮件脚本，可以把错误信息当作参数传进去
            "$EMAIL_SCRIPT" "Robot Task Board 运行失败 (TeaQL $LATEST_VERSION)" "在升级/运行新版本 $LATEST_VERSION 时 cargo test 失败，请检查日志。"
        else
            echo "$(date '+%Y-%m-%d %H:%M:%S') [Warning] 未找到邮件脚本或不可执行: $EMAIL_SCRIPT"
        fi
        
        exit 1
    else
        echo "$(date '+%Y-%m-%d %H:%M:%S') [Success] 运行成功！更新本地版本记录。"
        echo "$LATEST_VERSION" > "$VERSION_FILE"
    fi
else
    echo "$(date '+%Y-%m-%d %H:%M:%S') [Info] 版本未变化 ($LATEST_VERSION)，无需运行。"
fi
