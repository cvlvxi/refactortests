## Description

We want to find test cases that can be refactored where originally the Tests include Features

which then need to be enabled and initialized. This was done seperately via a function call 

e.g. `InitAndEnableFeature`


## Goal

- Refactor Test classes to default initialize the member with the features and remove any feature list initialization in places such as:
- constructor or other methods 

## Examples

- https://source.chromium.org/chromium/chromium/src/+/main:ash/test/ash_test_base.h;drc=5d4472724b3eaca3fe780e3d8b78cc5d452fd41c;l=78

## Requirements for test

<!-- - Inheritance from  : public testing::Test  -->
- Properties in their class with of type: ` base::test::ScopedFeatureList`
- At some point it calls the method: `InitAndEnableFeature`


## Tasks

### 1. Find a bunch of tests with the above requirement

### 2. Simply Regex parser
- Restricting it to classes
- Regex:


```c++

const message_center::Notification* GetPreviewNotification() {
  const message_center::NotificationList::Notifications notifications =
      message_center::MessageCenter::Get()->GetVisibleNotifications();
  for (const auto* notification : notifications) {
    if (notification->id() == kScreenCaptureNotificationId)
      return notification;
  }
  return nullptr;
}
```

### 3. Filter the classes by InitAndEnableFeature

### 4. Given a class with this there are rules:

1. Are there multiple calls to this? 
2
