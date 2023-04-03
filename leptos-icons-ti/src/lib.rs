
#[cfg_attr(
    feature = "serde",
    derive(
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Clone,
        Copy,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(
    not(feature = "serde"),
    derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)
)]
pub enum TiIcon {
    #[cfg(feature = "TiAdjustBrightness")]
    TiAdjustBrightness,
    #[cfg(feature = "TiAdjustContrast")]
    TiAdjustContrast,
    #[cfg(feature = "TiAnchor")]
    TiAnchor,
    #[cfg(feature = "TiAnchorOutline")]
    TiAnchorOutline,
    #[cfg(feature = "TiArchive")]
    TiArchive,
    #[cfg(feature = "TiArrowBack")]
    TiArrowBack,
    #[cfg(feature = "TiArrowBackOutline")]
    TiArrowBackOutline,
    #[cfg(feature = "TiArrowDown")]
    TiArrowDown,
    #[cfg(feature = "TiArrowDownOutline")]
    TiArrowDownOutline,
    #[cfg(feature = "TiArrowDownThick")]
    TiArrowDownThick,
    #[cfg(feature = "TiArrowForward")]
    TiArrowForward,
    #[cfg(feature = "TiArrowForwardOutline")]
    TiArrowForwardOutline,
    #[cfg(feature = "TiArrowLeft")]
    TiArrowLeft,
    #[cfg(feature = "TiArrowLeftOutline")]
    TiArrowLeftOutline,
    #[cfg(feature = "TiArrowLeftThick")]
    TiArrowLeftThick,
    #[cfg(feature = "TiArrowLoop")]
    TiArrowLoop,
    #[cfg(feature = "TiArrowLoopOutline")]
    TiArrowLoopOutline,
    #[cfg(feature = "TiArrowMaximise")]
    TiArrowMaximise,
    #[cfg(feature = "TiArrowMaximiseOutline")]
    TiArrowMaximiseOutline,
    #[cfg(feature = "TiArrowMinimise")]
    TiArrowMinimise,
    #[cfg(feature = "TiArrowMinimiseOutline")]
    TiArrowMinimiseOutline,
    #[cfg(feature = "TiArrowMove")]
    TiArrowMove,
    #[cfg(feature = "TiArrowMoveOutline")]
    TiArrowMoveOutline,
    #[cfg(feature = "TiArrowRepeat")]
    TiArrowRepeat,
    #[cfg(feature = "TiArrowRepeatOutline")]
    TiArrowRepeatOutline,
    #[cfg(feature = "TiArrowRight")]
    TiArrowRight,
    #[cfg(feature = "TiArrowRightOutline")]
    TiArrowRightOutline,
    #[cfg(feature = "TiArrowRightThick")]
    TiArrowRightThick,
    #[cfg(feature = "TiArrowShuffle")]
    TiArrowShuffle,
    #[cfg(feature = "TiArrowSortedDown")]
    TiArrowSortedDown,
    #[cfg(feature = "TiArrowSortedUp")]
    TiArrowSortedUp,
    #[cfg(feature = "TiArrowSync")]
    TiArrowSync,
    #[cfg(feature = "TiArrowSyncOutline")]
    TiArrowSyncOutline,
    #[cfg(feature = "TiArrowUnsorted")]
    TiArrowUnsorted,
    #[cfg(feature = "TiArrowUp")]
    TiArrowUp,
    #[cfg(feature = "TiArrowUpOutline")]
    TiArrowUpOutline,
    #[cfg(feature = "TiArrowUpThick")]
    TiArrowUpThick,
    #[cfg(feature = "TiAt")]
    TiAt,
    #[cfg(feature = "TiAttachment")]
    TiAttachment,
    #[cfg(feature = "TiAttachmentOutline")]
    TiAttachmentOutline,
    #[cfg(feature = "TiBackspace")]
    TiBackspace,
    #[cfg(feature = "TiBackspaceOutline")]
    TiBackspaceOutline,
    #[cfg(feature = "TiBatteryCharge")]
    TiBatteryCharge,
    #[cfg(feature = "TiBatteryFull")]
    TiBatteryFull,
    #[cfg(feature = "TiBatteryHigh")]
    TiBatteryHigh,
    #[cfg(feature = "TiBatteryLow")]
    TiBatteryLow,
    #[cfg(feature = "TiBatteryMid")]
    TiBatteryMid,
    #[cfg(feature = "TiBeaker")]
    TiBeaker,
    #[cfg(feature = "TiBeer")]
    TiBeer,
    #[cfg(feature = "TiBell")]
    TiBell,
    #[cfg(feature = "TiBook")]
    TiBook,
    #[cfg(feature = "TiBookmark")]
    TiBookmark,
    #[cfg(feature = "TiBriefcase")]
    TiBriefcase,
    #[cfg(feature = "TiBrush")]
    TiBrush,
    #[cfg(feature = "TiBusinessCard")]
    TiBusinessCard,
    #[cfg(feature = "TiCalculator")]
    TiCalculator,
    #[cfg(feature = "TiCalendar")]
    TiCalendar,
    #[cfg(feature = "TiCalendarOutline")]
    TiCalendarOutline,
    #[cfg(feature = "TiCamera")]
    TiCamera,
    #[cfg(feature = "TiCameraOutline")]
    TiCameraOutline,
    #[cfg(feature = "TiCancel")]
    TiCancel,
    #[cfg(feature = "TiCancelOutline")]
    TiCancelOutline,
    #[cfg(feature = "TiChartArea")]
    TiChartArea,
    #[cfg(feature = "TiChartAreaOutline")]
    TiChartAreaOutline,
    #[cfg(feature = "TiChartBar")]
    TiChartBar,
    #[cfg(feature = "TiChartBarOutline")]
    TiChartBarOutline,
    #[cfg(feature = "TiChartLine")]
    TiChartLine,
    #[cfg(feature = "TiChartLineOutline")]
    TiChartLineOutline,
    #[cfg(feature = "TiChartPie")]
    TiChartPie,
    #[cfg(feature = "TiChartPieOutline")]
    TiChartPieOutline,
    #[cfg(feature = "TiChevronLeft")]
    TiChevronLeft,
    #[cfg(feature = "TiChevronLeftOutline")]
    TiChevronLeftOutline,
    #[cfg(feature = "TiChevronRight")]
    TiChevronRight,
    #[cfg(feature = "TiChevronRightOutline")]
    TiChevronRightOutline,
    #[cfg(feature = "TiClipboard")]
    TiClipboard,
    #[cfg(feature = "TiCloudStorage")]
    TiCloudStorage,
    #[cfg(feature = "TiCloudStorageOutline")]
    TiCloudStorageOutline,
    #[cfg(feature = "TiCode")]
    TiCode,
    #[cfg(feature = "TiCodeOutline")]
    TiCodeOutline,
    #[cfg(feature = "TiCoffee")]
    TiCoffee,
    #[cfg(feature = "TiCog")]
    TiCog,
    #[cfg(feature = "TiCogOutline")]
    TiCogOutline,
    #[cfg(feature = "TiCompass")]
    TiCompass,
    #[cfg(feature = "TiContacts")]
    TiContacts,
    #[cfg(feature = "TiCreditCard")]
    TiCreditCard,
    #[cfg(feature = "TiCss3")]
    TiCss3,
    #[cfg(feature = "TiDatabase")]
    TiDatabase,
    #[cfg(feature = "TiDelete")]
    TiDelete,
    #[cfg(feature = "TiDeleteOutline")]
    TiDeleteOutline,
    #[cfg(feature = "TiDeviceDesktop")]
    TiDeviceDesktop,
    #[cfg(feature = "TiDeviceLaptop")]
    TiDeviceLaptop,
    #[cfg(feature = "TiDevicePhone")]
    TiDevicePhone,
    #[cfg(feature = "TiDeviceTablet")]
    TiDeviceTablet,
    #[cfg(feature = "TiDirections")]
    TiDirections,
    #[cfg(feature = "TiDivide")]
    TiDivide,
    #[cfg(feature = "TiDivideOutline")]
    TiDivideOutline,
    #[cfg(feature = "TiDocument")]
    TiDocument,
    #[cfg(feature = "TiDocumentAdd")]
    TiDocumentAdd,
    #[cfg(feature = "TiDocumentDelete")]
    TiDocumentDelete,
    #[cfg(feature = "TiDocumentText")]
    TiDocumentText,
    #[cfg(feature = "TiDownload")]
    TiDownload,
    #[cfg(feature = "TiDownloadOutline")]
    TiDownloadOutline,
    #[cfg(feature = "TiDropbox")]
    TiDropbox,
    #[cfg(feature = "TiEdit")]
    TiEdit,
    #[cfg(feature = "TiEject")]
    TiEject,
    #[cfg(feature = "TiEjectOutline")]
    TiEjectOutline,
    #[cfg(feature = "TiEquals")]
    TiEquals,
    #[cfg(feature = "TiEqualsOutline")]
    TiEqualsOutline,
    #[cfg(feature = "TiExport")]
    TiExport,
    #[cfg(feature = "TiExportOutline")]
    TiExportOutline,
    #[cfg(feature = "TiEye")]
    TiEye,
    #[cfg(feature = "TiEyeOutline")]
    TiEyeOutline,
    #[cfg(feature = "TiFeather")]
    TiFeather,
    #[cfg(feature = "TiFilm")]
    TiFilm,
    #[cfg(feature = "TiFilter")]
    TiFilter,
    #[cfg(feature = "TiFlag")]
    TiFlag,
    #[cfg(feature = "TiFlagOutline")]
    TiFlagOutline,
    #[cfg(feature = "TiFlash")]
    TiFlash,
    #[cfg(feature = "TiFlashOutline")]
    TiFlashOutline,
    #[cfg(feature = "TiFlowChildren")]
    TiFlowChildren,
    #[cfg(feature = "TiFlowMerge")]
    TiFlowMerge,
    #[cfg(feature = "TiFlowParallel")]
    TiFlowParallel,
    #[cfg(feature = "TiFlowSwitch")]
    TiFlowSwitch,
    #[cfg(feature = "TiFolder")]
    TiFolder,
    #[cfg(feature = "TiFolderAdd")]
    TiFolderAdd,
    #[cfg(feature = "TiFolderDelete")]
    TiFolderDelete,
    #[cfg(feature = "TiFolderOpen")]
    TiFolderOpen,
    #[cfg(feature = "TiGift")]
    TiGift,
    #[cfg(feature = "TiGlobe")]
    TiGlobe,
    #[cfg(feature = "TiGlobeOutline")]
    TiGlobeOutline,
    #[cfg(feature = "TiGroup")]
    TiGroup,
    #[cfg(feature = "TiGroupOutline")]
    TiGroupOutline,
    #[cfg(feature = "TiHeadphones")]
    TiHeadphones,
    #[cfg(feature = "TiHeart")]
    TiHeart,
    #[cfg(feature = "TiHeartFullOutline")]
    TiHeartFullOutline,
    #[cfg(feature = "TiHeartHalfOutline")]
    TiHeartHalfOutline,
    #[cfg(feature = "TiHeartOutline")]
    TiHeartOutline,
    #[cfg(feature = "TiHome")]
    TiHome,
    #[cfg(feature = "TiHomeOutline")]
    TiHomeOutline,
    #[cfg(feature = "TiHtml5")]
    TiHtml5,
    #[cfg(feature = "TiImage")]
    TiImage,
    #[cfg(feature = "TiImageOutline")]
    TiImageOutline,
    #[cfg(feature = "TiInfinity")]
    TiInfinity,
    #[cfg(feature = "TiInfinityOutline")]
    TiInfinityOutline,
    #[cfg(feature = "TiInfo")]
    TiInfo,
    #[cfg(feature = "TiInfoLarge")]
    TiInfoLarge,
    #[cfg(feature = "TiInfoLargeOutline")]
    TiInfoLargeOutline,
    #[cfg(feature = "TiInfoOutline")]
    TiInfoOutline,
    #[cfg(feature = "TiInputChecked")]
    TiInputChecked,
    #[cfg(feature = "TiInputCheckedOutline")]
    TiInputCheckedOutline,
    #[cfg(feature = "TiKey")]
    TiKey,
    #[cfg(feature = "TiKeyOutline")]
    TiKeyOutline,
    #[cfg(feature = "TiKeyboard")]
    TiKeyboard,
    #[cfg(feature = "TiLeaf")]
    TiLeaf,
    #[cfg(feature = "TiLightbulb")]
    TiLightbulb,
    #[cfg(feature = "TiLink")]
    TiLink,
    #[cfg(feature = "TiLinkOutline")]
    TiLinkOutline,
    #[cfg(feature = "TiLocation")]
    TiLocation,
    #[cfg(feature = "TiLocationArrow")]
    TiLocationArrow,
    #[cfg(feature = "TiLocationArrowOutline")]
    TiLocationArrowOutline,
    #[cfg(feature = "TiLocationOutline")]
    TiLocationOutline,
    #[cfg(feature = "TiLockClosed")]
    TiLockClosed,
    #[cfg(feature = "TiLockClosedOutline")]
    TiLockClosedOutline,
    #[cfg(feature = "TiLockOpen")]
    TiLockOpen,
    #[cfg(feature = "TiLockOpenOutline")]
    TiLockOpenOutline,
    #[cfg(feature = "TiMail")]
    TiMail,
    #[cfg(feature = "TiMap")]
    TiMap,
    #[cfg(feature = "TiMediaEject")]
    TiMediaEject,
    #[cfg(feature = "TiMediaEjectOutline")]
    TiMediaEjectOutline,
    #[cfg(feature = "TiMediaFastForward")]
    TiMediaFastForward,
    #[cfg(feature = "TiMediaFastForwardOutline")]
    TiMediaFastForwardOutline,
    #[cfg(feature = "TiMediaPause")]
    TiMediaPause,
    #[cfg(feature = "TiMediaPauseOutline")]
    TiMediaPauseOutline,
    #[cfg(feature = "TiMediaPlay")]
    TiMediaPlay,
    #[cfg(feature = "TiMediaPlayOutline")]
    TiMediaPlayOutline,
    #[cfg(feature = "TiMediaPlayReverse")]
    TiMediaPlayReverse,
    #[cfg(feature = "TiMediaPlayReverseOutline")]
    TiMediaPlayReverseOutline,
    #[cfg(feature = "TiMediaRecord")]
    TiMediaRecord,
    #[cfg(feature = "TiMediaRecordOutline")]
    TiMediaRecordOutline,
    #[cfg(feature = "TiMediaRewind")]
    TiMediaRewind,
    #[cfg(feature = "TiMediaRewindOutline")]
    TiMediaRewindOutline,
    #[cfg(feature = "TiMediaStop")]
    TiMediaStop,
    #[cfg(feature = "TiMediaStopOutline")]
    TiMediaStopOutline,
    #[cfg(feature = "TiMessage")]
    TiMessage,
    #[cfg(feature = "TiMessageTyping")]
    TiMessageTyping,
    #[cfg(feature = "TiMessages")]
    TiMessages,
    #[cfg(feature = "TiMicrophone")]
    TiMicrophone,
    #[cfg(feature = "TiMicrophoneOutline")]
    TiMicrophoneOutline,
    #[cfg(feature = "TiMinus")]
    TiMinus,
    #[cfg(feature = "TiMinusOutline")]
    TiMinusOutline,
    #[cfg(feature = "TiMortarBoard")]
    TiMortarBoard,
    #[cfg(feature = "TiNews")]
    TiNews,
    #[cfg(feature = "TiNotes")]
    TiNotes,
    #[cfg(feature = "TiNotesOutline")]
    TiNotesOutline,
    #[cfg(feature = "TiPen")]
    TiPen,
    #[cfg(feature = "TiPencil")]
    TiPencil,
    #[cfg(feature = "TiPhone")]
    TiPhone,
    #[cfg(feature = "TiPhoneOutline")]
    TiPhoneOutline,
    #[cfg(feature = "TiPi")]
    TiPi,
    #[cfg(feature = "TiPiOutline")]
    TiPiOutline,
    #[cfg(feature = "TiPin")]
    TiPin,
    #[cfg(feature = "TiPinOutline")]
    TiPinOutline,
    #[cfg(feature = "TiPipette")]
    TiPipette,
    #[cfg(feature = "TiPlane")]
    TiPlane,
    #[cfg(feature = "TiPlaneOutline")]
    TiPlaneOutline,
    #[cfg(feature = "TiPlug")]
    TiPlug,
    #[cfg(feature = "TiPlus")]
    TiPlus,
    #[cfg(feature = "TiPlusOutline")]
    TiPlusOutline,
    #[cfg(feature = "TiPointOfInterest")]
    TiPointOfInterest,
    #[cfg(feature = "TiPointOfInterestOutline")]
    TiPointOfInterestOutline,
    #[cfg(feature = "TiPower")]
    TiPower,
    #[cfg(feature = "TiPowerOutline")]
    TiPowerOutline,
    #[cfg(feature = "TiPrinter")]
    TiPrinter,
    #[cfg(feature = "TiPuzzle")]
    TiPuzzle,
    #[cfg(feature = "TiPuzzleOutline")]
    TiPuzzleOutline,
    #[cfg(feature = "TiRadar")]
    TiRadar,
    #[cfg(feature = "TiRadarOutline")]
    TiRadarOutline,
    #[cfg(feature = "TiRefresh")]
    TiRefresh,
    #[cfg(feature = "TiRefreshOutline")]
    TiRefreshOutline,
    #[cfg(feature = "TiRss")]
    TiRss,
    #[cfg(feature = "TiRssOutline")]
    TiRssOutline,
    #[cfg(feature = "TiScissors")]
    TiScissors,
    #[cfg(feature = "TiScissorsOutline")]
    TiScissorsOutline,
    #[cfg(feature = "TiShoppingBag")]
    TiShoppingBag,
    #[cfg(feature = "TiShoppingCart")]
    TiShoppingCart,
    #[cfg(feature = "TiSocialAtCircular")]
    TiSocialAtCircular,
    #[cfg(feature = "TiSocialDribbble")]
    TiSocialDribbble,
    #[cfg(feature = "TiSocialDribbbleCircular")]
    TiSocialDribbbleCircular,
    #[cfg(feature = "TiSocialFacebook")]
    TiSocialFacebook,
    #[cfg(feature = "TiSocialFacebookCircular")]
    TiSocialFacebookCircular,
    #[cfg(feature = "TiSocialFlickr")]
    TiSocialFlickr,
    #[cfg(feature = "TiSocialFlickrCircular")]
    TiSocialFlickrCircular,
    #[cfg(feature = "TiSocialGithub")]
    TiSocialGithub,
    #[cfg(feature = "TiSocialGithubCircular")]
    TiSocialGithubCircular,
    #[cfg(feature = "TiSocialGooglePlus")]
    TiSocialGooglePlus,
    #[cfg(feature = "TiSocialGooglePlusCircular")]
    TiSocialGooglePlusCircular,
    #[cfg(feature = "TiSocialInstagram")]
    TiSocialInstagram,
    #[cfg(feature = "TiSocialInstagramCircular")]
    TiSocialInstagramCircular,
    #[cfg(feature = "TiSocialLastFm")]
    TiSocialLastFm,
    #[cfg(feature = "TiSocialLastFmCircular")]
    TiSocialLastFmCircular,
    #[cfg(feature = "TiSocialLinkedin")]
    TiSocialLinkedin,
    #[cfg(feature = "TiSocialLinkedinCircular")]
    TiSocialLinkedinCircular,
    #[cfg(feature = "TiSocialPinterest")]
    TiSocialPinterest,
    #[cfg(feature = "TiSocialPinterestCircular")]
    TiSocialPinterestCircular,
    #[cfg(feature = "TiSocialSkype")]
    TiSocialSkype,
    #[cfg(feature = "TiSocialSkypeOutline")]
    TiSocialSkypeOutline,
    #[cfg(feature = "TiSocialTumbler")]
    TiSocialTumbler,
    #[cfg(feature = "TiSocialTumblerCircular")]
    TiSocialTumblerCircular,
    #[cfg(feature = "TiSocialTwitter")]
    TiSocialTwitter,
    #[cfg(feature = "TiSocialTwitterCircular")]
    TiSocialTwitterCircular,
    #[cfg(feature = "TiSocialVimeo")]
    TiSocialVimeo,
    #[cfg(feature = "TiSocialVimeoCircular")]
    TiSocialVimeoCircular,
    #[cfg(feature = "TiSocialYoutube")]
    TiSocialYoutube,
    #[cfg(feature = "TiSocialYoutubeCircular")]
    TiSocialYoutubeCircular,
    #[cfg(feature = "TiSortAlphabetically")]
    TiSortAlphabetically,
    #[cfg(feature = "TiSortAlphabeticallyOutline")]
    TiSortAlphabeticallyOutline,
    #[cfg(feature = "TiSortNumerically")]
    TiSortNumerically,
    #[cfg(feature = "TiSortNumericallyOutline")]
    TiSortNumericallyOutline,
    #[cfg(feature = "TiSpanner")]
    TiSpanner,
    #[cfg(feature = "TiSpannerOutline")]
    TiSpannerOutline,
    #[cfg(feature = "TiSpiral")]
    TiSpiral,
    #[cfg(feature = "TiStar")]
    TiStar,
    #[cfg(feature = "TiStarFullOutline")]
    TiStarFullOutline,
    #[cfg(feature = "TiStarHalf")]
    TiStarHalf,
    #[cfg(feature = "TiStarHalfOutline")]
    TiStarHalfOutline,
    #[cfg(feature = "TiStarOutline")]
    TiStarOutline,
    #[cfg(feature = "TiStarburst")]
    TiStarburst,
    #[cfg(feature = "TiStarburstOutline")]
    TiStarburstOutline,
    #[cfg(feature = "TiStopwatch")]
    TiStopwatch,
    #[cfg(feature = "TiSupport")]
    TiSupport,
    #[cfg(feature = "TiTabsOutline")]
    TiTabsOutline,
    #[cfg(feature = "TiTag")]
    TiTag,
    #[cfg(feature = "TiTags")]
    TiTags,
    #[cfg(feature = "TiThLarge")]
    TiThLarge,
    #[cfg(feature = "TiThLargeOutline")]
    TiThLargeOutline,
    #[cfg(feature = "TiThList")]
    TiThList,
    #[cfg(feature = "TiThListOutline")]
    TiThListOutline,
    #[cfg(feature = "TiThMenu")]
    TiThMenu,
    #[cfg(feature = "TiThMenuOutline")]
    TiThMenuOutline,
    #[cfg(feature = "TiThSmall")]
    TiThSmall,
    #[cfg(feature = "TiThSmallOutline")]
    TiThSmallOutline,
    #[cfg(feature = "TiThermometer")]
    TiThermometer,
    #[cfg(feature = "TiThumbsDown")]
    TiThumbsDown,
    #[cfg(feature = "TiThumbsOk")]
    TiThumbsOk,
    #[cfg(feature = "TiThumbsUp")]
    TiThumbsUp,
    #[cfg(feature = "TiTick")]
    TiTick,
    #[cfg(feature = "TiTickOutline")]
    TiTickOutline,
    #[cfg(feature = "TiTicket")]
    TiTicket,
    #[cfg(feature = "TiTime")]
    TiTime,
    #[cfg(feature = "TiTimes")]
    TiTimes,
    #[cfg(feature = "TiTimesOutline")]
    TiTimesOutline,
    #[cfg(feature = "TiTrash")]
    TiTrash,
    #[cfg(feature = "TiTree")]
    TiTree,
    #[cfg(feature = "TiUpload")]
    TiUpload,
    #[cfg(feature = "TiUploadOutline")]
    TiUploadOutline,
    #[cfg(feature = "TiUser")]
    TiUser,
    #[cfg(feature = "TiUserAdd")]
    TiUserAdd,
    #[cfg(feature = "TiUserAddOutline")]
    TiUserAddOutline,
    #[cfg(feature = "TiUserDelete")]
    TiUserDelete,
    #[cfg(feature = "TiUserDeleteOutline")]
    TiUserDeleteOutline,
    #[cfg(feature = "TiUserOutline")]
    TiUserOutline,
    #[cfg(feature = "TiVendorAndroid")]
    TiVendorAndroid,
    #[cfg(feature = "TiVendorApple")]
    TiVendorApple,
    #[cfg(feature = "TiVendorMicrosoft")]
    TiVendorMicrosoft,
    #[cfg(feature = "TiVideo")]
    TiVideo,
    #[cfg(feature = "TiVideoOutline")]
    TiVideoOutline,
    #[cfg(feature = "TiVolume")]
    TiVolume,
    #[cfg(feature = "TiVolumeDown")]
    TiVolumeDown,
    #[cfg(feature = "TiVolumeMute")]
    TiVolumeMute,
    #[cfg(feature = "TiVolumeUp")]
    TiVolumeUp,
    #[cfg(feature = "TiWarning")]
    TiWarning,
    #[cfg(feature = "TiWarningOutline")]
    TiWarningOutline,
    #[cfg(feature = "TiWatch")]
    TiWatch,
    #[cfg(feature = "TiWaves")]
    TiWaves,
    #[cfg(feature = "TiWavesOutline")]
    TiWavesOutline,
    #[cfg(feature = "TiWeatherCloudy")]
    TiWeatherCloudy,
    #[cfg(feature = "TiWeatherDownpour")]
    TiWeatherDownpour,
    #[cfg(feature = "TiWeatherNight")]
    TiWeatherNight,
    #[cfg(feature = "TiWeatherPartlySunny")]
    TiWeatherPartlySunny,
    #[cfg(feature = "TiWeatherShower")]
    TiWeatherShower,
    #[cfg(feature = "TiWeatherSnow")]
    TiWeatherSnow,
    #[cfg(feature = "TiWeatherStormy")]
    TiWeatherStormy,
    #[cfg(feature = "TiWeatherSunny")]
    TiWeatherSunny,
    #[cfg(feature = "TiWeatherWindy")]
    TiWeatherWindy,
    #[cfg(feature = "TiWeatherWindyCloudy")]
    TiWeatherWindyCloudy,
    #[cfg(feature = "TiWiFi")]
    TiWiFi,
    #[cfg(feature = "TiWiFiOutline")]
    TiWiFiOutline,
    #[cfg(feature = "TiWine")]
    TiWine,
    #[cfg(feature = "TiWorld")]
    TiWorld,
    #[cfg(feature = "TiWorldOutline")]
    TiWorldOutline,
    #[cfg(feature = "TiZoom")]
    TiZoom,
    #[cfg(feature = "TiZoomIn")]
    TiZoomIn,
    #[cfg(feature = "TiZoomInOutline")]
    TiZoomInOutline,
    #[cfg(feature = "TiZoomOut")]
    TiZoomOut,
    #[cfg(feature = "TiZoomOutOutline")]
    TiZoomOutOutline,
    #[cfg(feature = "TiZoomOutline")]
    TiZoomOutline,
}

#[allow(unused)]
#[allow(non_snake_case)]
pub fn LeptosTiIcon(
    #[allow(unused)]
    cx: leptos::Scope,
    #[allow(unused)]
    icon: TiIcon,
    #[allow(unused)]
    width: Option<String>,
    #[allow(unused)]
    height: Option<String>,
    #[allow(unused)]
    class: Option<String>,
    #[allow(unused)]
    style: Option<String>,
    #[allow(unused)]
    title: Option<String>,
) -> leptos::View {
    #[allow(unused)]
    let width = width.unwrap_or_else(|| String::from("1em"));
    #[allow(unused)]
    let height = height.unwrap_or_else(|| String::from("1em"));
    #[allow(unused)]
    let class = class.unwrap_or_else(|| String::from(""));
    #[allow(unused)]
    let style = style.unwrap_or_else(|| String::from(""));
    match icon {
        #[cfg(feature = "TiAdjustBrightness")]
        TiIcon::TiAdjustBrightness => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 6.934l1-2.934c.072-.213.078-.452 0-.682-.188-.553-.789-.848-1.341-.659-.553.189-.847.788-.659 1.341l1 2.934zM4 11c-.213-.072-.452-.078-.682 0-.553.188-.848.789-.659 1.341.189.553.788.847 1.341.659l2.934-1-2.934-1zM12 17.066l-1 2.934c-.072.213-.078.452 0 .682.188.553.789.848 1.341.659.553-.189.847-.788.659-1.341l-1-2.934zM21.341 11.657c-.188-.553-.788-.848-1.341-.659l-2.934 1 2.934 1c.213.072.452.078.682 0 .552-.188.847-.789.659-1.341zM5.636 7.05l2.781 1.367-1.367-2.781c-.1-.202-.265-.375-.482-.482-.524-.258-1.157-.042-1.415.482-.257.523-.041 1.157.483 1.414zM5.153 17.432c-.257.523-.041 1.156.482 1.414.523.257 1.157.041 1.414-.482l1.367-2.781-2.781 1.367c-.201.099-.374.263-.482.482zM18.363 16.949l-2.781-1.367 1.367 2.781c.1.202.264.375.482.482.523.257 1.156.041 1.414-.482s.042-1.157-.482-1.414zM18.844 6.566c.258-.524.042-1.157-.481-1.415-.523-.257-1.157-.041-1.414.482l-1.369 2.783 2.782-1.368c.202-.1.375-.264.482-.482zM12 7.5c-2.481 0-4.5 2.019-4.5 4.5s2.019 4.5 4.5 4.5 4.5-2.019 4.5-4.5-2.019-4.5-4.5-4.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiAdjustBrightness".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiAdjustContrast")]
        TiIcon::TiAdjustContrast => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 4c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zm0 14c-3.314 0-6-2.686-6-6s2.686-6 6-6 6 2.686 6 6-2.686 6-6 6zM12 7v10c2.757 0 5-2.243 5-5s-2.243-5-5-5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiAdjustContrast".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiAnchor")]
        TiIcon::TiAnchor => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 13.5c-.553 0-1 .447-1 1 0 2.414-1.721 4.434-4 4.898v-7.398h4c.553 0 1-.447 1-1s-.447-1-1-1h-4v-1.184c1.162-.413 2-1.511 2-2.816 0-1.657-1.343-3-3-3s-3 1.343-3 3c0 1.305.838 2.403 2 2.816v1.184h-4c-.553 0-1 .447-1 1s.447 1 1 1h4v7.398c-2.279-.465-4-2.484-4-4.898 0-.553-.447-1-1-1s-1 .447-1 1c0 3.859 3.141 7 7 7s7-3.141 7-7c0-.553-.447-1-1-1zm-6-8.5c.551 0 1 .449 1 1s-.449 1-1 1-1-.449-1-1 .449-1 1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiAnchor".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiAnchorOutline")]
        TiIcon::TiAnchorOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <circle cx=\"12\" cy=\"6\" r=\"1\" />\n  <path d=\"M19.793 12.096c.134-.34.207-.709.207-1.096 0-1.654-1.346-3-3-3h-.422c.273-.619.422-1.297.422-2 0-2.757-2.243-5-5-5s-5 2.243-5 5c0 .703.149 1.381.422 2h-.422c-1.654 0-3 1.346-3 3 0 .387.073.756.207 1.096-.732.548-1.207 1.422-1.207 2.404 0 4.963 4.037 9 9 9s9-4.037 9-9c0-.982-.475-1.856-1.207-2.404zm-7.793 9.404c-3.859 0-7-3.141-7-7 0-.553.447-1 1-1s1 .447 1 1c0 2.414 1.721 4.434 4 4.898v-7.398h-4c-.553 0-1-.447-1-1s.447-1 1-1h4v-1.184c-1.162-.413-2-1.511-2-2.816 0-1.657 1.343-3 3-3s3 1.343 3 3c0 1.305-.838 2.403-2 2.816v1.184h4c.553 0 1 .447 1 1s-.447 1-1 1h-4v7.398c2.279-.465 4-2.484 4-4.898 0-.553.447-1 1-1s1 .447 1 1c0 3.859-3.141 7-7 7zm-4.679-8.5h2.679v4.962c-1.207-.701-2-2.009-2-3.462 0-.597-.263-1.133-.679-1.5zm9.358 0c-.416.367-.679.903-.679 1.5 0 1.453-.793 2.761-2 3.462v-4.962h2.679z\" />\n  <circle cx=\"12\" cy=\"6\" r=\"1\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiAnchorOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArchive")]
        TiIcon::TiArchive => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M13 12h-3c-.276 0-.5.224-.5.5s.224.5.5.5h3c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM20 5h-17c-.553 0-1 .448-1 1s.447 1 1 1h17c.553 0 1-.448 1-1s-.447-1-1-1zM18 8h-13c-.553 0-1 .448-1 1v8c0 1.654 1.346 3 3 3h9c1.654 0 3-1.346 3-3v-8c0-.552-.447-1-1-1zm-2 10h-9c-.552 0-1-.449-1-1v-7h11v7c0 .551-.448 1-1 1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArchive".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowBack")]
        TiIcon::TiArrowBack => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 9.059v-2.559c0-.256-.098-.512-.293-.708-.195-.195-.451-.292-.707-.292s-.512.097-.707.292l-6.293 6.208 6.293 6.207c.195.195.451.293.707.293s.512-.098.707-.293.293-.452.293-.707v-2.489c2.75.068 5.755.566 8 3.989v-1c0-4.633-3.5-8.443-8-8.941z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowBack".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowBackOutline")]
        TiIcon::TiArrowBackOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.164 19.547c-1.641-2.5-3.669-3.285-6.164-3.484v1.437c0 .534-.208 1.036-.586 1.414-.756.756-2.077.751-2.823.005l-6.293-6.207c-.191-.189-.298-.444-.298-.713s.107-.524.298-.712l6.288-6.203c.754-.755 2.073-.756 2.829.001.377.378.585.88.585 1.414v1.704c4.619.933 8 4.997 8 9.796v1c0 .442-.29.832-.714.958-.095.027-.19.042-.286.042-.331 0-.646-.165-.836-.452zm-7.141-5.536c2.207.056 4.638.394 6.758 2.121-.768-3.216-3.477-5.702-6.893-6.08-.504-.056-.888-.052-.888-.052v-3.497l-5.576 5.496 5.576 5.5v-3.499l1.023.011z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowBackOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowDown")]
        TiIcon::TiArrowDown => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.707 13.293c-.391-.391-1.023-.391-1.414 0l-2.293 2.293v-7.586c0-.552-.447-1-1-1s-1 .448-1 1v7.586l-2.293-2.293c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l4.707 4.707 4.707-4.707c.391-.391.391-1.023 0-1.414z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowDown".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowDownOutline")]
        TiIcon::TiArrowDownOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 21.312l-7.121-7.121c-1.17-1.17-1.17-3.073 0-4.242 1.094-1.094 2.978-1.138 4.121-.115v-4.834c0-1.654 1.346-3 3-3s3 1.346 3 3v4.834c1.143-1.023 3.027-.979 4.121.115 1.17 1.169 1.17 3.072 0 4.242l-7.121 7.121zm-5-10.242c-.268 0-.518.104-.707.293-.391.39-.391 1.023 0 1.414l5.707 5.707 5.707-5.707c.391-.391.391-1.024 0-1.414-.379-.379-1.035-.379-1.414 0l-3.293 3.293v-9.656c0-.551-.448-1-1-1s-1 .449-1 1v9.656l-3.293-3.293c-.189-.189-.439-.293-.707-.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowDownOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowDownThick")]
        TiIcon::TiArrowDownThick => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.414 10.656c-.781-.781-2.047-.781-2.828 0l-1.586 1.586v-7.242c0-1.105-.896-2-2-2-1.105 0-2 .895-2 2v7.242l-1.586-1.586c-.781-.781-2.047-.781-2.828 0s-.781 2.047 0 2.828l6.414 6.414 6.414-6.414c.781-.781.781-2.046 0-2.828z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowDownThick".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowForward")]
        TiIcon::TiArrowForward => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 5.499c-.256 0-.512.097-.707.292-.195.196-.293.452-.293.708v2.559c-4.5.498-8 4.309-8 8.941v1c2.245-3.423 5.25-3.92 8-3.989v2.489c0 .255.098.512.293.707s.451.293.707.293.512-.098.707-.293l6.293-6.207-6.293-6.208c-.195-.195-.451-.292-.707-.292z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowForward".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowForwardOutline")]
        TiIcon::TiArrowForwardOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M4 19.999c-.096 0-.191-.015-.286-.042-.424-.126-.714-.516-.714-.958v-1c0-4.8 3.381-8.864 8-9.796v-1.704c0-.534.208-1.036.585-1.414.756-.757 2.075-.756 2.829-.001l6.288 6.203c.191.188.298.443.298.712s-.107.524-.298.712l-6.293 6.207c-.746.746-2.067.751-2.823-.005-.378-.378-.586-.88-.586-1.414v-1.437c-2.495.201-4.523.985-6.164 3.484-.19.288-.505.453-.836.453zm8-5.989l1-.01v3.499l5.576-5.5-5.576-5.496v3.497s-.384-.004-.891.052c-3.416.378-6.125 2.864-6.892 6.08 2.121-1.728 4.551-2.066 6.783-2.122z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowForwardOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowLeft")]
        TiIcon::TiArrowLeft => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 11h-7.586l2.293-2.293c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-4.707 4.707 4.707 4.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-2.293-2.293h7.586c.552 0 1-.448 1-1s-.448-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowLeft".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowLeftOutline")]
        TiIcon::TiArrowLeftOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.928 21c-.801 0-1.555-.312-2.121-.879l-7.121-7.121 7.121-7.121c1.133-1.134 3.109-1.134 4.242 0 .566.564.879 1.317.879 2.119 0 .746-.27 1.451-.764 2.002h4.836c1.654 0 3 1.346 3 3s-1.346 3-3 3h-4.836c.493.549.764 1.252.764 1.998.002.802-.312 1.557-.879 2.124-.567.566-1.32.878-2.121.878zm-6.414-8l5.707 5.707c.379.378 1.035.378 1.414 0 .189-.189.293-.441.293-.708s-.104-.517-.291-.705l-3.295-3.294h9.658c.552 0 1-.449 1-1s-.448-1-1-1h-9.658l3.293-3.293c.189-.189.293-.441.293-.708s-.104-.517-.292-.705c-.381-.38-1.036-.379-1.415-.001l-5.707 5.707z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowLeftOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowLeftThick")]
        TiIcon::TiArrowLeftThick => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 11h-7.244l1.586-1.586c.781-.781.781-2.049 0-2.828-.781-.781-2.047-.781-2.828 0l-6.414 6.414 6.414 6.414c.39.391.902.586 1.414.586s1.023-.195 1.414-.586c.781-.781.781-2.049 0-2.828l-1.586-1.586h7.244c1.104 0 2-.896 2-2 0-1.105-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowLeftThick".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowLoop")]
        TiIcon::TiArrowLoop => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.5 8h-2.086l1.293-1.293c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-3.707 3.707 3.707 3.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-1.293-1.293h2.086c1.379 0 2.5 1.346 2.5 3s-1.346 3-3 3h-8c-1.654 0-3-1.346-3-3s1.346-3 3-3c.553 0 1-.448 1-1s-.447-1-1-1c-2.757 0-5 2.243-5 5s2.243 5 5 5h8c2.757 0 5-2.243 5-5s-2.019-5-4.5-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowLoop".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowLoopOutline")]
        TiIcon::TiArrowLoopOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.994 7.187l.006-.187c0-.801-.312-1.555-.879-2.121-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-2.883 2.883c-.531-.474-1.23-.762-1.996-.762h-1c-3.859 0-7 3.14-7 7s3.141 7 7 7h9c3.859 0 7-3.14 7-7 0-3.306-2.14-6.084-5.006-6.813zm-1.994 11.813h-9c-2.757 0-5-2.243-5-5s2.243-5 5-5h1c.553 0 1 .448 1 1s-.447 1-1 1h-1c-1.654 0-3 1.346-3 3s1.346 3 3 3h9c1.654 0 3-1.346 3-3s-1.121-3-2.5-3h-2.086l1.293 1.293c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-3.707-3.707 3.707-3.707c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-1.293 1.293h2.086c2.481 0 4.5 2.243 4.5 5s-2.243 5-5 5zm.749-6.971c.7.164 1.251 1 1.251 1.971 0 1.103-.897 2-2 2h-9c-1.103 0-2-.897-2-2s.897-2 2-2h1c.856 0 1.588-.541 1.873-1.299l3.713 3.713c.378.378.88.586 1.414.586s1.036-.208 1.414-.586.586-.88.586-1.414c0-.345-.087-.677-.251-.971z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowLoopOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowMaximise")]
        TiIcon::TiArrowMaximise => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15 4c-.553 0-1 .448-1 1s.447 1 1 1h1.586l-3.293 3.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.293-3.293v1.586c0 .552.447 1 1 1s1-.448 1-1v-5h-5zM9.293 13.293l-3.293 3.293v-1.586c0-.552-.447-1-1-1s-1 .448-1 1v4.999h.996l4.004.001c.552 0 1-.448 1-1s-.447-1-1-1h-1.586l3.293-3.292c.391-.391.391-1.023 0-1.414s-1.023-.392-1.414-.001zM7 12c.552 0 1-.448 1-1v-3h3c.553 0 1-.448 1-1s-.447-1-1-1h-4.999l-.001 5c0 .552.447 1 1 1zM17 12c-.553 0-1 .448-1 1v3h-3c-.553 0-1 .448-1 1s.447 1 1 1h5v-5c0-.552-.447-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowMaximise".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowMaximiseOutline")]
        TiIcon::TiArrowMaximiseOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 3h-5.243c-1.302 0-2.401.838-2.815 2h-6.942v7.061l.012.12c-1.167.41-2.012 1.512-2.012 2.819v7h7c1.311 0 2.593-.826 3-2h7v-7.061l-.012-.12c1.167-.41 2.012-1.512 2.012-2.819v-7h-2zm-2 15h-5c-.553 0-1-.448-1-1s.447-1 1-1h3v-3.061c0-.552.447-1 1-1s1 .448 1 1v5.061zm-11-11h5.061c.553 0 1 .448 1 1s-.447 1-1 1h-3.061v3.061c0 .552-.448 1-1 1-.553 0-1-.448-1-1v-5.061zm13 3c0 .552-.447 1-1 1s-1-.448-1-1v-1.586l-3.293 3.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l3.293-3.293h-1.586c-.553 0-1-.448-1-1s.447-1 1-1h5v5zm-10 10h-5v-5c0-.552.447-1 1-1s1 .448 1 1v1.586l3.293-3.293c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-3.293 3.293h1.586c.553 0 1 .448 1 1s-.448 1-1 1zm2.414-7.414c-.378-.378-.88-.586-1.414-.586-.367 0-.716.105-1.023.289l.023-.228v-2.061h2.061l.229-.023c-.186.307-.29.656-.29 1.023 0 .534.208 1.036.586 1.414s.88.586 1.414.586c.367 0 .716-.105 1.023-.289l-.023.228v2.061h-1.939c-.122 0-.24.015-.356.036.189-.31.295-.664.295-1.036 0-.534-.208-1.036-.586-1.414z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowMaximiseOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowMinimise")]
        TiIcon::TiArrowMinimise => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M6.121 13c-.553 0-1 .448-1 1s.447 1 1 1h1.465l-3.293 3.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.414-3.414v1.707c0 .552.447 1 1 1s.879-.448.879-1v-5h-4.879zM7 11c.552 0 1-.448 1-1v-2h2c.553 0 1-.448 1-1s-.447-1-1-1h-3.999l-.001 4c0 .552.447 1 1 1zM17 13c-.553 0-1 .448-1 1v2h-2c-.553 0-1 .448-1 1s.447 1 1 1h4v-4c0-.552-.447-1-1-1zM18.293 4.293l-3.293 3.293v-1.586c0-.552-.447-1-1-1s-1 .448-1 1v5h5c.552 0 1-.448 1-1s-.447-1-1-1h-1.586l3.293-3.292c.391-.391.391-1.023 0-1.414s-1.023-.392-1.414-.001z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowMinimise".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowMinimiseOutline")]
        TiIcon::TiArrowMinimiseOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22 6c0-.801-.312-1.555-.879-2.121-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-.883.883c-.531-.474-1.23-.762-1.996-.762-.919 0-1.732.424-2.283 1.077-.212-.047-.431-.077-.656-.077h-6.061v6.06c0 .255.042.499.102.736-.598.549-.98 1.33-.98 2.204 0 .735.266 1.409.705 1.931l-.947.948c-.568.566-.88 1.32-.88 2.121s.312 1.555.879 2.121c.566.567 1.32.879 2.121.879.539 0 1.334-.152 2.061-.879l.903-.919c.535.495 1.251.798 2.036.798.934 0 1.758-.437 2.309-1.107.241.063.49.107.752.107h5.939v-6.061c0-.226-.029-.444-.077-.656.653-.55 1.077-1.364 1.077-2.283 0-.766-.288-1.465-.762-1.996l.883-.883c.567-.566.879-1.32.879-2.121zm-15 1h4c.553 0 1 .448 1 1s-.447 1-1 1h-2v2c0 .552-.448 1-1 1-.553 0-1-.448-1-1v-4zm12.707-.293l-3.293 3.293h1.586c.553 0 1 .448 1 1s-.448 1-1 1h-5v-5c0-.552.447-1 1-1s1 .448 1 1v1.586l3.293-3.293c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414zm-7.707 11.293c0 .552-.447 1-1 1s-1-.448-1-1v-1.707l-3.354 3.414c-.195.195-.39.293-.646.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l3.293-3.293h-1.465c-.553 0-1-.448-1-1s.447-1 1-1h4.879v5zm0-6h-2.278c.173-.295.278-.634.278-1v-1h1.061c.342 0 .658-.094.939-.245v2.245zm1 1h2.713c-.433.094-.713.33-.713.939v1.061h-.939c-.391 0-.752.117-1.061.311v-2.311zm.061 4c0-.552.447-1 1-1h1.939v-2c0-.552.447-1 1-1s1 .448 1 1v4h-3.939c-.553 0-1-.448-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowMinimiseOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowMove")]
        TiIcon::TiArrowMove => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.707 8.293c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l1.293 1.293h-4.586v-4.586l1.293 1.293c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-3.707-3.707-3.707 3.707c-.391.391-.391 1.023 0 1.414s1.023.391 1.414 0l1.293-1.293v4.586h-4.586l1.293-1.293c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-3.707 3.707 3.707 3.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-1.293-1.293h4.586v4.586l-1.293-1.293c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l3.707 3.707 3.707-3.707c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-1.293 1.293v-4.586h4.586l-1.293 1.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.707-3.707-3.707-3.707z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowMove".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowMoveOutline")]
        TiIcon::TiArrowMoveOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22.828 10.586l-9.414-9.414c-.391-.391-.902-.586-1.414-.586s-1.023.195-1.414.586l-9.414 9.414c-.781.779-.781 2.047 0 2.828l9.414 9.414c.391.391.902.586 1.414.586s1.023-.195 1.414-.586l9.414-9.414c.781-.781.781-2.049 0-2.828zm-5.828 5.414c-.256 0-.512-.098-.707-.293-.391-.391-.391-1.023 0-1.414l1.293-1.293h-4.586v4.586l1.293-1.293c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-3.707 3.707-3.707-3.707c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l1.293 1.293v-4.586h-4.586l1.293 1.293c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-3.707-3.707 3.707-3.707c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-1.293 1.293h4.586v-4.586l-1.293 1.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l3.707-3.707 3.707 3.707c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-1.293-1.293v4.586h4.586l-1.293-1.293c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l3.707 3.707-3.707 3.707c-.195.195-.451.293-.707.293zm-1.732-2c-.175.301-.268.643-.268 1-.357 0-.699.093-1 .268v-1.268h1.268zm-6.536 0h1.268v1.268c-.301-.175-.643-.268-1-.268 0-.357-.093-.699-.268-1zm0-4c.175-.301.268-.643.268-1 .357 0 .699-.093 1-.268v1.268h-1.268zm6.536 0h-1.268v-1.268c.301.175.643.268 1 .268 0 .357.093.699.268 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowMoveOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowRepeat")]
        TiIcon::TiArrowRepeat => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.5 7h-2.086l1.293-1.293c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-3.707 3.707 3.707 3.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-1.293-1.293h2.086c1.379 0 2.5 1.346 2.5 3s-1.346 3-3 3c-.553 0-1 .448-1 1s.447 1 1 1c2.757 0 5-2.243 5-5s-2.019-5-4.5-5zM8.293 12.293c-.391.391-.391 1.023 0 1.414l1.293 1.293h-2.086c-1.379 0-2.5-1.346-2.5-3s1.346-3 3-3c.553 0 1-.448 1-1s-.447-1-1-1c-2.757 0-5 2.243-5 5s2.019 5 4.5 5h2.086l-1.293 1.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.707-3.707-3.707-3.707c-.391-.391-1.023-.391-1.414 0z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowRepeat".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowRepeatOutline")]
        TiIcon::TiArrowRepeatOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.994 7.187l.006-.187c0-.801-.312-1.555-.879-2.121-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-2.892 2.891c-.53-.473-1.221-.77-1.987-.77h-1c-3.859 0-7 3.14-7 7 0 3.306 2.14 6.084 5.006 6.813l-.006.187c0 .801.312 1.555.879 2.121.566.567 1.32.879 2.121.879s1.555-.312 2.121-.879l2.892-2.891c.53.473 1.221.77 1.987.77h1c3.859 0 7-3.14 7-7 0-3.306-2.14-6.084-5.006-6.813zm-1.994 11.813h-1c-.553 0-1-.448-1-1s.447-1 1-1h1c1.654 0 3-1.346 3-3s-1.121-3-2.5-3h-2.086l1.293 1.293c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-3.707-3.707 3.707-3.707c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-1.293 1.293h2.086c2.481 0 4.5 2.243 4.5 5s-2.243 5-5 5zm.749-6.971c.7.164 1.251 1 1.251 1.971 0 1.103-.897 2-2 2h-1c-.857 0-1.584.544-1.868 1.304l-3.718-3.718c-.378-.378-.88-.586-1.414-.586s-1.036.208-1.414.586-.586.88-.586 1.414c0 .345.087.677.251.971-.7-.164-1.251-1-1.251-1.971 0-1.103.897-2 2-2h1c.857 0 1.584-.544 1.868-1.304l3.718 3.718c.378.378.88.586 1.414.586s1.036-.208 1.414-.586.586-.88.586-1.414c0-.345-.087-.677-.251-.971zm-7.749-2.029c0 .552-.447 1-1 1h-1c-1.654 0-3 1.346-3 3s1.121 3 2.5 3h2.086l-1.293-1.293c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l3.707 3.707-3.707 3.707c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l1.293-1.293h-2.086c-2.481 0-4.5-2.243-4.5-5s2.243-5 5-5h1c.553 0 1 .448 1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowRepeatOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowRight")]
        TiIcon::TiArrowRight => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13.293 7.293c-.391.391-.391 1.023 0 1.414l2.293 2.293h-7.586c-.552 0-1 .448-1 1s.448 1 1 1h7.586l-2.293 2.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l4.707-4.707-4.707-4.707c-.391-.391-1.023-.391-1.414 0z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowRight".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowRightOutline")]
        TiIcon::TiArrowRightOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 21c-.801 0-1.555-.312-2.121-.879s-.88-1.321-.879-2.123c0-.746.271-.998.764-1.998h-4.836c-1.654 0-3-1.347-3-3 0-1.654 1.346-3 3-3h4.836c-.494-1-.764-1.255-.764-2.001.001-.802.312-1.554.88-2.121 1.132-1.132 3.108-1.133 4.241.001l7.121 7.121-7.121 7.121c-.566.567-1.32.879-2.121.879zm-7.072-9c-.552 0-1 .449-1 1s.448 1 1 1h9.658l-3.293 3.293c-.189.189-.293.439-.293.706 0 .269.104.519.293.708.379.378 1.035.378 1.414 0l5.707-5.707-5.707-5.707c-.379-.378-1.035-.378-1.414 0-.189.189-.293.439-.293.706 0 .268.104.519.293.708l3.293 3.293h-9.658z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowRightOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowRightThick")]
        TiIcon::TiArrowRightThick => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.586 6.586c-.781.779-.781 2.047 0 2.828l1.586 1.586h-7.244c-1.104 0-2 .895-2 2 0 1.104.896 2 2 2h7.244l-1.586 1.586c-.781.779-.781 2.047 0 2.828.391.391.902.586 1.414.586s1.023-.195 1.414-.586l6.414-6.414-6.414-6.414c-.781-.781-2.047-.781-2.828 0z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowRightThick".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowShuffle")]
        TiIcon::TiArrowShuffle => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M4 9h3.5c.736 0 1.393.391 1.851 1.001.325-.604.729-1.163 1.191-1.662-.803-.823-1.866-1.339-3.042-1.339h-3.5c-.553 0-1 .448-1 1s.447 1 1 1zM11.685 12.111c.551-1.657 2.256-3.111 3.649-3.111h1.838l-1.293 1.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.707-3.707-3.707-3.707c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l1.293 1.293h-1.838c-2.274 0-4.711 1.967-5.547 4.479l-.472 1.411c-.641 1.926-2.072 3.11-2.815 3.11h-2.5c-.553 0-1 .448-1 1s.447 1 1 1h2.5c1.837 0 3.863-1.925 4.713-4.479l.472-1.41zM15.879 13.293c-.391.391-.391 1.023 0 1.414l1.293 1.293h-2.338c-1.268 0-2.33-.891-2.691-2.108-.256.75-.627 1.499-1.09 2.185.886 1.162 2.243 1.923 3.781 1.923h2.338l-1.293 1.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.707-3.707-3.707-3.707c-.391-.391-1.023-.391-1.414 0z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowShuffle".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowSortedDown")]
        TiIcon::TiArrowSortedDown => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M5.8 9.7l6.2 6.3 6.2-6.3c.2-.2.3-.5.3-.7s-.1-.5-.3-.7c-.2-.2-.4-.3-.7-.3h-11c-.3 0-.5.1-.7.3-.2.2-.3.4-.3.7s.1.5.3.7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowSortedDown".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowSortedUp")]
        TiIcon::TiArrowSortedUp => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.2 13.3l-6.2-6.3-6.2 6.3c-.2.2-.3.5-.3.7s.1.5.3.7c.2.2.4.3.7.3h11c.3 0 .5-.1.7-.3.2-.2.3-.5.3-.7s-.1-.5-.3-.7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowSortedUp".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowSync")]
        TiIcon::TiArrowSync => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.5 12.473c0-1.948-.618-3.397-2.066-4.844-.391-.39-1.023-.39-1.414 0-.391.391-.391 1.024 0 1.415 1.079 1.078 1.48 2.007 1.48 3.429 0 1.469-.572 2.85-1.611 3.888-1.004 1.003-2.078 1.502-3.428 1.593l1.246-1.247c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-3.707 3.707 3.707 3.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-1.337-1.336c1.923-.082 3.542-.792 4.933-2.181 1.417-1.416 2.197-3.299 2.197-5.303zM6.5 12.5c0-1.469.572-2.85 1.611-3.889 1.009-1.009 2.092-1.508 3.457-1.594l-1.275 1.275c-.391.391-.391 1.023 0 1.414.195.196.451.294.707.294s.512-.098.707-.293l3.707-3.707-3.707-3.707c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l1.311 1.311c-1.914.086-3.525.796-4.907 2.179-1.417 1.416-2.197 3.299-2.197 5.303 0 1.948.618 3.397 2.066 4.844.195.195.451.292.707.292s.512-.098.707-.293c.391-.391.391-1.024 0-1.415-1.079-1.077-1.48-2.006-1.48-3.428z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowSync".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowSyncOutline")]
        TiIcon::TiArrowSyncOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.5 12.473c0-2.495-.818-4.426-2.653-6.259-.309-.309-.676-.533-1.073-.682l-.946-.946-3.707-3.707c-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879c-.567.566-.879 1.32-.879 2.121 0 .277.037.549.11.809-1.029.461-1.974 1.12-2.827 1.974-1.795 1.793-2.783 4.178-2.783 6.717 0 2.495.818 4.426 2.653 6.259.299.298.652.521 1.034.669l.985.986 3.707 3.707c.566.567 1.32.879 2.121.879s1.555-.312 2.121-.879c.567-.566.879-1.32.879-2.121 0-.286-.04-.566-.117-.834 1.031-.461 1.978-1.121 2.833-1.975 1.796-1.794 2.784-4.18 2.784-6.718zm-9.13 7.484l1.337 1.336c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-3.707-3.707 3.707-3.707c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-1.247 1.247c1.351-.091 2.425-.59 3.428-1.593 1.039-1.038 1.611-2.419 1.611-3.888 0-1.422-.401-2.351-1.48-3.429-.391-.391-.391-1.023 0-1.415.195-.195.451-.293.708-.293.256 0 .512.098.707.292 1.448 1.447 2.066 2.896 2.066 4.844 0 2.004-.78 3.887-2.197 5.303-1.39 1.39-3.01 2.1-4.933 2.182zm-.766-14.939l-1.311-1.311c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l3.707 3.707-3.707 3.707c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l1.275-1.275c-1.365.086-2.448.584-3.456 1.593-1.04 1.039-1.612 2.42-1.612 3.889 0 1.422.401 2.351 1.48 3.429.391.391.391 1.023 0 1.415-.195.195-.452.293-.708.293s-.512-.098-.707-.292c-1.447-1.448-2.065-2.897-2.065-4.845 0-2.004.78-3.887 2.197-5.303 1.382-1.383 2.993-2.093 4.907-2.179zm-2.916 10.204c-.888-.887-1.188-1.574-1.188-2.722 0-1.202.468-2.332 1.318-3.181l.187-.179c.033.481.236.93.581 1.274.378.378.88.586 1.414.586s1.036-.208 1.414-.586l2.339-2.339c-.078.596.104 1.219.56 1.675.888.887 1.188 1.574 1.188 2.722 0 1.202-.468 2.332-1.318 3.181l-.188.181c-.039-.472-.241-.91-.579-1.248-.38-.378-.882-.586-1.416-.586s-1.036.208-1.414.586l-2.342 2.342c.089-.605-.093-1.242-.556-1.706z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowSyncOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowUnsorted")]
        TiIcon::TiArrowUnsorted => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.2 9.3l-6.2-6.3-6.2 6.3c-.2.2-.3.4-.3.7s.1.5.3.7c.2.2.4.3.7.3h11c.3 0 .5-.1.7-.3.2-.2.3-.5.3-.7s-.1-.5-.3-.7zM5.8 14.7l6.2 6.3 6.2-6.3c.2-.2.3-.5.3-.7s-.1-.5-.3-.7c-.2-.2-.4-.3-.7-.3h-11c-.3 0-.5.1-.7.3-.2.2-.3.5-.3.7s.1.5.3.7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowUnsorted".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowUp")]
        TiIcon::TiArrowUp => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 5.586l-4.707 4.707c-.391.391-.391 1.023 0 1.414s1.023.391 1.414 0l2.293-2.293v7.586c0 .552.447 1 1 1s1-.448 1-1v-7.586l2.293 2.293c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-4.707-4.707z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowUp".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowUpOutline")]
        TiIcon::TiArrowUpOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 21c-1.654 0-3-1.346-3-3v-4.764c-1.143 1.024-3.025.979-4.121-.115-1.17-1.169-1.17-3.073 0-4.242l7.121-7.121 7.121 7.121c1.17 1.169 1.17 3.073 0 4.242-1.094 1.095-2.979 1.14-4.121.115v4.764c0 1.654-1.346 3-3 3zm-1-12.586v9.586c0 .551.448 1 1 1s1-.449 1-1v-9.586l3.293 3.293c.379.378 1.035.378 1.414 0 .391-.391.391-1.023 0-1.414l-5.707-5.707-5.707 5.707c-.391.391-.391 1.023 0 1.414.379.378 1.035.378 1.414 0l3.293-3.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiArrowUpOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiArrowUpThick")]
        TiIcon::TiArrowUpThick => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 3.172l-6.414 6.414c-.781.781-.781 2.047 0 2.828s2.047.781 2.828 0l1.586-1.586v7.242c0 1.104.895 2 2 2 1.104 0 2-.896 2-2v-7.242l1.586 1.586c.391.391.902.586 1.414.586s1.023-.195 1.414-.586c.781-.781.781-2.047 0-2.828l-6.414-6.414z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiArrowUpThick".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiAt")]
        TiIcon::TiAt => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 4c-4.411 0-8 3.589-8 8s3.589 8 8 8c1.616 0 3.172-.479 4.499-1.384.456-.312.574-.934.263-1.39-.311-.457-.932-.572-1.39-.263-.994.679-2.16 1.037-3.372 1.037-3.309 0-6-2.691-6-6s2.691-6 6-6 6 2.691 6 6v.5c0 .552-.448 1-1 1s-1-.448-1-1v-3c0-.553-.447-1-1-1-.441 0-.805.29-.938.688-.58-.427-1.289-.688-2.062-.688-1.93 0-3.5 1.57-3.5 3.5s1.57 3.5 3.5 3.5c1.045 0 1.975-.47 2.616-1.199.548.723 1.408 1.199 2.384 1.199 1.654 0 3-1.346 3-3v-.5c0-4.411-3.589-8-8-8zm0 9.5c-.827 0-1.5-.673-1.5-1.5s.673-1.5 1.5-1.5 1.5.673 1.5 1.5-.673 1.5-1.5 1.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiAt".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiAttachment")]
        TiIcon::TiAttachment => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.364 6.635c-1.561-1.559-4.1-1.559-5.658 0l-4.534 4.535c-.473.473-.733 1.1-.733 1.77 0 .668.261 1.295.732 1.768.487.486 1.128.73 1.769.73.64 0 1.279-.242 1.767-.73l2.122-2.121c.391-.395.586-.904.586-1.414 0-.512-.195-1.023-.586-1.414l-3.536 3.535c-.193.195-.511.195-.708-.002-.127-.127-.146-.275-.146-.352 0-.078.019-.227.146-.354l4.535-4.537c.778-.779 2.048-.779 2.83 0 .779.779.779 2.049 0 2.828l-4.537 4.537-2.535 2.535c-.779.779-2.049.779-2.828 0-.78-.779-.78-2.049 0-2.828l.095-.096c-.451-.6-.702-1.359-.702-2.125l-.807.807c-1.56 1.559-1.56 4.098 0 5.656.779.779 1.804 1.17 2.828 1.17s2.049-.391 2.828-1.17l7.072-7.072c1.56-1.559 1.56-4.096 0-5.656z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiAttachment".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiAttachmentOutline")]
        TiIcon::TiAttachmentOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15.534 4.466c1.024 0 2.05.39 2.829 1.169 1.561 1.561 1.561 4.098 0 5.656l-7.071 7.072c-.778.779-1.804 1.17-2.828 1.17s-2.049-.391-2.828-1.17c-1.56-1.559-1.56-4.098 0-5.656l.807-.807c-.004.805.25 1.524.701 2.125l-.094.096c-.78.779-.78 2.049 0 2.828.39.39.901.584 1.414.584s1.024-.195 1.414-.584l2.535-2.535 4.537-4.537c.778-.779.778-2.049 0-2.828-.392-.39-.904-.584-1.417-.584-.512 0-1.023.195-1.413.584l-4.535 4.537c-.128.127-.146.275-.146.354 0 .076.019.226.146.353.099.099.228.147.356.147.127 0 .254-.049.352-.146l2.122-2.121 1.414-1.414c.392.392.586.902.586 1.414 0 .511-.194 1.021-.584 1.41l-2.124 2.125c-.486.487-1.127.729-1.768.729s-1.28-.244-1.769-.729c-.472-.474-.731-1.101-.731-1.769 0-.67.261-1.297.732-1.77l4.534-4.535c.779-.779 1.805-1.168 2.829-1.168m0-2c-1.604 0-3.11.623-4.242 1.755l-7.069 7.073c-1.133 1.131-1.757 2.638-1.757 4.242s.624 3.11 1.757 4.243c1.131 1.132 2.639 1.755 4.241 1.755s3.11-.624 4.242-1.757l7.071-7.071c1.133-1.131 1.757-2.638 1.757-4.242 0-1.603-.623-3.11-1.755-4.241-1.133-1.134-2.64-1.757-4.245-1.757z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiAttachmentOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBackspace")]
        TiIcon::TiBackspace => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.5 5h-10c-1.266 0-2.834.807-3.57 1.837l-2.61 3.653-1.199 1.679c-.121.175-.122.492.003.664l1.188 1.664 2.619 3.667c.735 1.029 2.302 1.836 3.569 1.836h10c1.379 0 2.5-1.122 2.5-2.5v-10c0-1.378-1.121-2.5-2.5-2.5zm-2.293 9.793c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-2.293-2.293-2.293 2.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l2.293-2.293-2.293-2.293c-.391-.391-.391-1.023 0-1.414s1.023-.391 1.414 0l2.293 2.293 2.293-2.293c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414l-2.293 2.293 2.293 2.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBackspace".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBackspaceOutline")]
        TiIcon::TiBackspaceOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 21h-10c-1.436 0-3.145-.88-3.977-2.046l-2.619-3.667-1.188-1.661c-.246-.344-.249-.894-.008-1.241l1.204-1.686 2.608-3.653c.835-1.167 2.546-2.046 3.98-2.046h10c1.654 0 3 1.346 3 3v10c0 1.654-1.346 3-3 3zm-15.771-8.001l.806 1.125 2.618 3.667c.451.633 1.57 1.209 2.348 1.209h10c.552 0 1-.45 1-1.001v-9.999c0-.551-.448-1-1-1h-10c-.776 0-1.897.576-2.351 1.209l-2.608 3.652-.813 1.138zM13.707 13l2.646-2.646c.194-.194.194-.512 0-.707-.195-.194-.513-.194-.707 0l-2.646 2.646-2.646-2.646c-.195-.194-.513-.194-.707 0-.195.195-.195.513 0 .707l2.646 2.646-2.646 2.646c-.195.195-.195.513 0 .707.097.098.225.147.353.147s.256-.049.354-.146l2.646-2.647 2.646 2.646c.098.098.226.147.354.147s.256-.049.354-.146c.194-.194.194-.512 0-.707l-2.647-2.647z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiBackspaceOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBatteryCharge")]
        TiIcon::TiBatteryCharge => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M5 10v6h11v-6h-11zm5.83 4.908l-1.21-1.908-2.62.428 3.223-2.324 1.175 1.896 2.602-.43-3.17 2.338zM19 10c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3 1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm-2 6c0 .552-.449 1-1 1h-11c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1h11c.551 0 1 .448 1 1v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBatteryCharge".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBatteryFull")]
        TiIcon::TiBatteryFull => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM6 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM15 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM12 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM19 10c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3 1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm-2 6c0 .552-.449 1-1 1h-11c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1h11c.551 0 1 .448 1 1v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBatteryFull".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBatteryHigh")]
        TiIcon::TiBatteryHigh => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM6 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM12 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM19 10c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3 1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm-2 6c0 .552-.449 1-1 1h-11c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1h11c.551 0 1 .448 1 1v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBatteryHigh".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBatteryLow")]
        TiIcon::TiBatteryLow => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M6 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM19 10c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3 1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm-2 6c0 .552-.449 1-1 1h-11c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1h11c.551 0 1 .448 1 1v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBatteryLow".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBatteryMid")]
        TiIcon::TiBatteryMid => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM6 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM19 10c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3 1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm-2 6c0 .552-.449 1-1 1h-11c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1h11c.551 0 1 .448 1 1v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBatteryMid".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBeaker")]
        TiIcon::TiBeaker => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.445 16.809l-2.64-9.809h1.195c.552 0 1-.448 1-1s-.448-1-1-1h-12c-.552 0-1 .448-1 1s.448 1 1 1h1.135c-.013.176-.048.402-.121.671l-2.459 9.138c-.218.809-.074 1.623.393 2.231.466.61 1.214.96 2.052.96h10c.838 0 1.586-.35 2.055-.959.466-.609.609-1.423.39-2.232zm-4.713-9.809l1.352 5.018-.084-.018h-8l-.084.018 1.029-3.826c.084-.312.173-.744.192-1.192h5.595zm2.734 10.824c-.087.114-.252.176-.466.176h-10c-.214 0-.379-.062-.466-.176-.086-.113-.104-.289-.048-.496l1.197-4.45c.088.073.195.122.317.122h8c.122 0 .229-.049.316-.121l1.197 4.45c.057.206.04.382-.047.495z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBeaker".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBeer")]
        TiIcon::TiBeer => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M10 16.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5v-6c0-.275.225-.5.5-.5s.5.225.5.5v6zM12 16.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5v-6c0-.275.225-.5.5-.5s.5.225.5.5v6zM14 16.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5v-6c0-.275.225-.5.5-.5s.5.225.5.5v6zM18.5 6h-.5v-1c0-1.104-.896-2-2-2h-9c-1.104 0-2 .896-2 2v13c0 1.656 1.344 3 3 3h7c1.656 0 3-1.344 3-3h.5c1.93 0 3.5-1.57 3.5-3.5v-5c0-1.93-1.57-3.5-3.5-3.5zm-11.5-1h9v1h-4.444l-.118.332c-.164.458-.663.73-1.117.648l-.348-.058-.173.307c-.267.475-.765.771-1.3.771-.827 0-1.5-.673-1.5-1.5v-1.5zm9 13c0 .552-.448 1-1 1h-7c-.552 0-1-.448-1-1v-9.51c.419.317.936.51 1.5.51.784 0 1.521-.376 1.989-1 .728 0 1.383-.391 1.736-1h3.775v11zm4-3.5c0 .827-.673 1.5-1.5 1.5h-1.5v-8h1.5c.827 0 1.5.673 1.5 1.5v5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBeer".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBell")]
        TiIcon::TiBell => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.715 17.301c-.017-.018-1.717-1.854-1.73-6.32-.009-2.607-1.69-4.824-4.019-5.641l.034-.34c0-1.103-.896-2-2-2s-2 .897-2 2l.034.338c-2.336.816-4.019 3.036-4.019 5.646 0 4.462-1.711 6.296-1.721 6.306-.287.286-.374.716-.22 1.091s.521.619.926.619h3.143c.447 1.72 1.999 3 3.857 3s3.41-1.28 3.857-3h3.143c.4 0 .758-.243.915-.61s.076-.799-.2-1.089zm-7.715-10.301c2.189 0 3.978 1.789 3.984 3.987.002.728.046 1.396.118 2.013h-8.2c.071-.617.113-1.286.113-2.016.001-2.196 1.788-3.984 3.985-3.984zm0 13c-.737 0-1.375-.405-1.722-1h3.443c-.346.595-.984 1-1.721 1zm-5.186-3c.352-.736.705-1.731.938-3h8.502c.234 1.269.588 2.264.938 3h-10.378z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBell".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBook")]
        TiIcon::TiBook => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 3h-11c-.265 0-.52.105-.707.293l-3 3-.057.062c-.139.165-.225.373-.235.6l-.001.053v10.992c0 1.654 1.346 3 3 3h9c1.304 0 2.416-.836 2.829-2h.671c1.402 0 2.5-1.317 2.5-3v-10c0-1.654-1.346-3-3-3zm-12 16c-.551 0-1-.448-1-1v-10h2v11h-1zm10-1c0 .552-.449 1-1 1h-7v-11h7c.551 0 1 .448 1 1v9zm3-2c0 .62-.324 1-.5 1h-.5v-8c0-1.654-1.346-3-3-3h-8.586l1-1h10.586c.551 0 1 .448 1 1v10z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBook".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBookmark")]
        TiIcon::TiBookmark => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 2h-8c-1.654 0-3 1.346-3 3v14c0 .514.104.946.308 1.285.564.935 1.815 1.008 2.813.008l3.172-3.172c.375-.374 1.039-.374 1.414 0l3.172 3.172c.491.491 1.002.74 1.52.74.797 0 1.601-.629 1.601-2.033v-14c0-1.654-1.346-3-3-3zm-8 2h8c.551 0 1 .449 1 1v9.905l-2.451-2.247c-1.406-1.289-3.693-1.288-5.099 0l-2.45 2.247v-9.905c0-.551.449-1 1-1zm6.121 11.707c-.565-.565-1.318-.876-2.121-.876s-1.556.312-2.121.876l-2.879 2.879v-2.324l3.126-2.866c1.033-.947 2.714-.947 3.747 0l3.127 2.866v2.324l-2.879-2.879z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBookmark".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBriefcase")]
        TiIcon::TiBriefcase => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M18 7c0-1.654-1.346-3-3-3h-6c-1.654 0-3 1.346-3 3-1.654 0-3 1.346-3 3v7c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-7c0-1.654-1.346-3-3-3zm-9-1h6c.551 0 1 .449 1 1h-8c0-.551.449-1 1-1zm10 11c0 .551-.449 1-1 1h-12c-.551 0-1-.449-1-1v-1h14v1zm-14-2v-5c0-.551.449-1 1-1h12c.551 0 1 .449 1 1v5h-14zM13 12h-2c-.55 0-1 .45-1 1s.45 1 1 1h2c.55 0 1-.45 1-1s-.45-1-1-1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBriefcase".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBrush")]
        TiIcon::TiBrush => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.177 3.823c-.382-.383-.894-.586-1.415-.586-.25 0-.503.047-.744.144-4.449 1.787-7.792 4.76-10.517 9.357-.102.172-.163.355-.209.542-1.38.215-2.6.903-3.442 1.993-.916 1.185-1.295 2.695-1.066 4.254l.216 1.473 1.473.217c.293.043.589.064.88.064 2.743 0 4.949-1.909 5.367-4.564.188-.047.371-.115.544-.218 4.598-2.728 7.571-6.069 9.355-10.517.298-.743.123-1.593-.442-2.159zm-14.824 15.458c-.192 0-.389-.016-.59-.044-.309-2.104 1.055-3.81 3-4.021l1.021 1.021c-.192 1.76-1.605 3.044-3.431 3.044zm4.89-4.502l-1.021-1.021c.38-.641.774-1.233 1.178-1.804.027.041 1.639 1.653 1.639 1.653-.568.401-1.158.794-1.796 1.172zm2.608-1.773s-1.821-1.801-1.879-1.824c2.147-2.784 4.651-4.685 7.791-5.943-1.255 3.127-3.144 5.623-5.912 7.767z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBrush".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiBusinessCard")]
        TiIcon::TiBusinessCard => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M20 20h-16c-1.654 0-3-1.346-3-3v-10c0-1.654 1.346-3 3-3h16c1.654 0 3 1.346 3 3v10c0 1.654-1.346 3-3 3zm-16-14c-.551 0-1 .449-1 1v10c0 .551.449 1 1 1h16c.551 0 1-.449 1-1v-10c0-.551-.449-1-1-1h-16zM10 15h-4c-.553 0-1-.448-1-1s.447-1 1-1h4c.553 0 1 .448 1 1s-.447 1-1 1zM10 11h-4c-.553 0-1-.448-1-1s.447-1 1-1h4c.553 0 1 .448 1 1s-.447 1-1 1z\" />\n  <circle cx=\"16\" cy=\"10.5\" r=\"2\" />\n  <path d=\"M16 13.356c-1.562 0-2.5.715-2.5 1.429 0 .357.938.715 2.5.715 1.466 0 2.5-.357 2.5-.715 0-.714-.98-1.429-2.5-1.429z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiBusinessCard".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCalculator")]
        TiIcon::TiCalculator => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 21h-8c-1.7 0-3-1.3-3-3v-12c0-1.7 1.3-3 3-3h8c1.7 0 3 1.3 3 3v12c0 1.7-1.3 3-3 3zm-8-16c-.6 0-1 .4-1 1v12c0 .6.4 1 1 1h8c.6 0 1-.4 1-1v-12c0-.6-.4-1-1-1h-8z\" />\n<circle cx=\"10\" cy=\"11\" r=\"1\" />\n<circle cx=\"13\" cy=\"11\" r=\"1\" />\n<circle cx=\"16\" cy=\"11\" r=\"1\" />\n<circle cx=\"10\" cy=\"14\" r=\"1\" />\n<circle cx=\"13\" cy=\"14\" r=\"1\" />\n<circle cx=\"16\" cy=\"14\" r=\"1\" />\n<circle cx=\"10\" cy=\"17\" r=\"1\" />\n<circle cx=\"13\" cy=\"17\" r=\"1\" />\n<circle cx=\"16\" cy=\"17\" r=\"1\" />\n<path d=\"M16 7v1h-6v-1h6m1-1h-8v3h8v-3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCalculator".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCalendar")]
        TiIcon::TiCalendar => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 6.184v-.184c0-1.657-1.343-3-3-3s-3 1.343-3 3h-2c0-1.657-1.343-3-3-3s-3 1.343-3 3v.184c-1.161.415-2 1.514-2 2.816v9c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-9c0-1.302-.839-2.401-2-2.816zm-4-.184c0-.552.447-1 1-1s1 .448 1 1v2c0 .552-.447 1-1 1s-1-.448-1-1v-2zm-8 0c0-.552.447-1 1-1s1 .448 1 1v2c0 .552-.447 1-1 1s-1-.448-1-1v-2zm12 12c0 .551-.448 1-1 1h-12c-.552 0-1-.449-1-1v-6h14v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCalendar".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCalendarOutline")]
        TiIcon::TiCalendarOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 6.184v-.184c0-1.657-1.343-3-3-3s-3 1.343-3 3h-2c0-1.657-1.343-3-3-3s-3 1.343-3 3v.184c-1.161.415-2 1.514-2 2.816v9c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-9c0-1.302-.839-2.401-2-2.816zm-4-.184c0-.552.447-1 1-1s1 .448 1 1v2c0 .552-.447 1-1 1s-1-.448-1-1v-2zm-8 0c0-.552.447-1 1-1s1 .448 1 1v2c0 .552-.447 1-1 1s-1-.448-1-1v-2zm12 12c0 .551-.448 1-1 1h-12c-.552 0-1-.449-1-1v-6h14v6zm0-7h-14v-2c0-.551.448-1 1-1 0 1.104.896 2 2 2s2-.896 2-2h4c0 1.104.896 2 2 2s2-.896 2-2c.552 0 1 .449 1 1v2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiCalendarOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCamera")]
        TiIcon::TiCamera => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 6h-1.586l-1-1c-.579-.579-1.595-1-2.414-1h-4c-.819 0-1.835.421-2.414 1l-1 1h-1.586c-1.654 0-3 1.346-3 3v8c0 1.654 1.346 3 3 3h14c1.654 0 3-1.346 3-3v-8c0-1.654-1.346-3-3-3zm-7 10c-1.933 0-3.5-1.568-3.5-3.5 0-1.934 1.567-3.5 3.5-3.5s3.5 1.566 3.5 3.5c0 1.932-1.567 3.5-3.5 3.5zm6-4.701c-.719 0-1.3-.58-1.3-1.299s.581-1.301 1.3-1.301 1.3.582 1.3 1.301-.581 1.299-1.3 1.299z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCamera".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCameraOutline")]
        TiIcon::TiCameraOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 20h-14c-1.654 0-3-1.346-3-3v-8c0-1.654 1.346-3 3-3h1.586l1-1c.579-.579 1.596-1 2.414-1h4c.818 0 1.835.421 2.414 1l1 1h1.586c1.654 0 3 1.346 3 3v8c0 1.654-1.346 3-3 3zm-14-12c-.552 0-1 .448-1 1v8c0 .552.448 1 1 1h14c.552 0 1-.448 1-1v-8c0-.552-.448-1-1-1h-2c-.266 0-.52-.105-.707-.293l-1.293-1.293c-.201-.201-.715-.414-1-.414h-4c-.285 0-.799.213-1 .414l-1.293 1.293c-.187.188-.441.293-.707.293h-2zM12 10c1.379 0 2.5 1.121 2.5 2.5s-1.121 2.5-2.5 2.5-2.5-1.121-2.5-2.5 1.121-2.5 2.5-2.5m0-1c-1.934 0-3.5 1.566-3.5 3.5 0 1.932 1.566 3.5 3.5 3.5s3.5-1.568 3.5-3.5c0-1.934-1.566-3.5-3.5-3.5zM18 8.699c-.719 0-1.3.582-1.3 1.301s.581 1.299 1.3 1.299 1.3-.58 1.3-1.299-.581-1.301-1.3-1.301z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCameraOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCancel")]
        TiIcon::TiCancel => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 4c-4.411 0-8 3.589-8 8s3.589 8 8 8 8-3.589 8-8-3.589-8-8-8zm-5 8c0-.832.224-1.604.584-2.295l6.711 6.711c-.691.36-1.463.584-2.295.584-2.757 0-5-2.243-5-5zm9.416 2.295l-6.711-6.711c.691-.36 1.463-.584 2.295-.584 2.757 0 5 2.243 5 5 0 .832-.224 1.604-.584 2.295z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCancel".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCancelOutline")]
        TiIcon::TiCancelOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 20.5c-4.688 0-8.5-3.812-8.5-8.5s3.812-8.5 8.497-8.5c4.69 0 8.503 3.812 8.503 8.5s-3.812 8.5-8.5 8.5zm0-15c-3.586 0-6.5 2.916-6.5 6.5s2.916 6.5 6.5 6.5 6.5-2.916 6.5-6.5-2.916-6.5-6.5-6.5zM12.003 8.5c1.929 0 3.497 1.57 3.497 3.5 0 .206-.02.412-.057.615l-4.057-4.059c.203-.036.408-.056.614-.056m.003-1c-.882 0-1.696.262-2.39.697l6.188 6.188c.438-.692.699-1.508.699-2.387 0-2.48-2.014-4.498-4.497-4.498zM8.557 11.384l4.059 4.06c-.204.036-.409.056-.616.056-1.93 0-3.5-1.57-3.5-3.502 0-.206.02-.412.057-.614m-.358-1.773c-.435.694-.699 1.508-.699 2.387 0 2.486 2.016 4.502 4.5 4.502.879 0 1.693-.264 2.387-.699l-6.188-6.19z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCancelOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartArea")]
        TiIcon::TiChartArea => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20 6c0-.587-.257-1.167-.75-1.562-.863-.69-2.121-.551-2.812.312l-2.789 3.486-2.449-1.836c-.864-.648-2.087-.493-2.762.351l-4 5c-.294.368-.438.811-.438 1.249v3h16v-10zM20 19h-16c-.552 0-1 .447-1 1s.448 1 1 1h16c.552 0 1-.447 1-1s-.448-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiChartArea".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartAreaOutline")]
        TiIcon::TiChartAreaOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20 17h-16c-.552 0-1-.447-1-1v-3c0-.68.234-1.346.658-1.874l4-5c.98-1.226 2.885-1.469 4.143-.524l1.674 1.254 2.185-2.729c.57-.717 1.424-1.127 2.341-1.127.679 0 1.343.232 1.873.657.716.572 1.126 1.426 1.126 2.343v10c0 .553-.448 1-1 1zm-15-2h14v-9c0-.307-.137-.59-.375-.779-.227-.183-.465-.221-.624-.221-.306 0-.591.137-.782.376l-2.789 3.485c-.337.423-.949.5-1.381.176l-2.449-1.837c-.422-.316-1.055-.233-1.381.176l-4 5c-.181.228-.219.464-.219.624v2zM20 21h-16c-.552 0-1-.447-1-1s.448-1 1-1h16c.552 0 1 .447 1 1s-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiChartAreaOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartBar")]
        TiIcon::TiChartBar => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14 4c0-1.105-.896-2-2-2s-2 .895-2 2v12h4v-12zM19 8c0-1.105-.896-2-2-2s-2 .895-2 2v8h4v-8zM9 11c0-1.105-.896-2-2-2s-2 .895-2 2v5h4v-5zM19 19h-14c-.553 0-1 .447-1 1s.447 1 1 1h14c.553 0 1-.447 1-1s-.447-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiChartBar".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartBarOutline")]
        TiIcon::TiChartBarOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 5c-.771 0-1.468.301-2 .779v-1.779c0-1.654-1.346-3-3-3s-3 1.346-3 3v4.779c-.532-.478-1.229-.779-2-.779-1.654 0-3 1.346-3 3v6h16v-9c0-1.654-1.346-3-3-3zm-5-2c.551 0 1 .448 1 1v11h-2v-11c0-.552.449-1 1-1zm-4 12h-2v-4c0-.552.449-1 1-1s1 .448 1 1v4zm10 0h-2v-7c0-.552.449-1 1-1s1 .448 1 1v7zM19 21h-14c-.552 0-1-.447-1-1s.448-1 1-1h14c.552 0 1 .447 1 1s-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiChartBarOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartLine")]
        TiIcon::TiChartLine => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M4.75 15.561c.369.294.811.439 1.248.439.588 0 1.168-.258 1.563-.752l2.789-3.486 2.45 1.838c.864.648 2.088.492 2.762-.352l4-5c.69-.861.55-2.121-.312-2.811-.863-.689-2.121-.551-2.812.312l-2.789 3.486-2.449-1.835c-.864-.648-2.087-.494-2.762.35l-4 5c-.69.863-.549 2.121.312 2.811zM5 21h14c.553 0 1-.447 1-1s-.447-1-1-1h-14c-.553 0-1 .447-1 1s.447 1 1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiChartLine".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartLineOutline")]
        TiIcon::TiChartLineOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M5.999 17c-.677 0-1.342-.234-1.873-.658-.626-.501-1.019-1.215-1.107-2.011-.089-.796.138-1.58.639-2.206l4-5c.978-1.225 2.883-1.471 4.143-.523l1.674 1.254 2.184-2.729c.571-.716 1.425-1.127 2.342-1.127.679 0 1.343.232 1.873.657.626.501 1.021 1.216 1.108 2.013s-.14 1.58-.641 2.204l-4 5c-.977 1.226-2.882 1.471-4.143.526l-1.674-1.256-2.184 2.729c-.57.716-1.424 1.127-2.341 1.127zm4.001-9c-.306 0-.591.137-.781.374l-4 5.001c-.167.208-.243.471-.213.734.03.266.161.504.369.67.228.183.465.221.624.221.306 0 .591-.137.782-.376l3.395-4.244 3.224 2.42c.42.316 1.056.231 1.381-.176l4-5.001c.167-.208.242-.469.213-.734-.03-.266-.161-.504-.369-.67-.227-.182-.464-.22-.624-.22-.306 0-.591.137-.782.376l-3.395 4.242-3.224-2.417c-.175-.132-.382-.2-.6-.2zM19 21h-14c-.552 0-1-.447-1-1s.448-1 1-1h14c.552 0 1 .447 1 1s-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiChartLineOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartPie")]
        TiIcon::TiChartPie => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.614 13.98l4.908 4.922c.39.391.99.36 1.286-.106.88-1.394 1.393-3.044 1.393-4.815 0-2.131-.741-4.086-1.972-5.631l-5.615 5.63zM9 14.396v-7.355c-3.391.487-6 3.405-6 6.939 0 3.876 3.134 7.02 7 7.02 1.572 0 3.018-.526 4.186-1.403l-5.186-5.201zM16.331 6.213c.39-.391.365-.999-.089-1.313-1.253-.868-2.695-1.479-4.251-1.765-.544-.1-.991.312-.991.865v7.56l5.331-5.347z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiChartPie".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChartPieOutline")]
        TiIcon::TiChartPieOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.227 7.609l.557-.559c.396-.396.607-.943.582-1.504-.026-.561-.286-1.084-.717-1.443-2.129-1.775-4.711-2.848-7.469-3.097l-.18-.006c-.497 0-.979.186-1.35.523-.414.379-.65.915-.65 1.477v2.229c-3.657.865-6.333 4.188-6.333 8.006 0 4.547 3.688 8.244 8.224 8.244 1.594 0 3.11-.479 4.441-1.345.277.142.583.226.9.226l.109-.004c.569-.03 1.098-.305 1.453-.75 1.421-1.781 2.204-4.019 2.204-6.297.002-2.032-.625-4.027-1.771-5.7zm-7.336 11.87c-3.438 0-6.224-2.793-6.224-6.244 0-3.137 2.317-5.729 5.333-6.164v6.408l4.609 4.754c-1.037.779-2.322 1.246-3.718 1.246zm.109-7.454v-9.025c2.411.218 4.607 1.173 6.366 2.641l-6.366 6.384zm.214 1.269l5.019-5.028c1.104 1.385 1.769 3.141 1.769 5.043 0 1.914-.664 3.666-1.769 5.051l-5.019-5.066z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiChartPieOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChevronLeft")]
        TiIcon::TiChevronLeft => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14.414 5.586c-.78-.781-2.048-.781-2.828 0l-6.415 6.414 6.415 6.414c.39.391.902.586 1.414.586s1.024-.195 1.414-.586c.781-.781.781-2.047 0-2.828l-3.585-3.586 3.585-3.586c.781-.781.781-2.047 0-2.828z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiChevronLeft".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChevronLeftOutline")]
        TiIcon::TiChevronLeftOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 20c-.802 0-1.555-.312-2.122-.879l-7.121-7.121 7.122-7.121c1.133-1.133 3.11-1.133 4.243 0 .566.566.878 1.32.878 2.121s-.312 1.555-.879 2.122l-2.878 2.878 2.878 2.879c.567.566.879 1.32.879 2.121s-.312 1.555-.879 2.122c-.566.566-1.319.878-2.121.878zm-6.415-8l5.708 5.707c.378.378 1.037.377 1.414 0 .189-.189.293-.439.293-.707s-.104-.518-.293-.707l-4.292-4.293 4.292-4.293c.189-.189.293-.44.293-.707s-.104-.518-.293-.707c-.378-.379-1.037-.378-1.414-.001l-5.708 5.708z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiChevronLeftOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChevronRight")]
        TiIcon::TiChevronRight => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8.586 5.586c-.781.781-.781 2.047 0 2.828l3.585 3.586-3.585 3.586c-.781.781-.781 2.047 0 2.828.39.391.902.586 1.414.586s1.024-.195 1.414-.586l6.415-6.414-6.415-6.414c-.78-.781-2.048-.781-2.828 0z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiChevronRight".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiChevronRightOutline")]
        TiIcon::TiChevronRightOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10 20c-.802 0-1.555-.312-2.122-.879-.566-.566-.878-1.32-.878-2.121s.312-1.555.879-2.122l2.878-2.878-2.878-2.879c-.567-.566-.879-1.32-.879-2.121s.312-1.555.879-2.122c1.133-1.132 3.109-1.133 4.243.001l7.121 7.121-7.122 7.121c-.566.567-1.319.879-2.121.879zm0-14c-.268 0-.518.104-.707.292-.189.19-.293.441-.293.708s.104.518.293.707l4.292 4.293-4.292 4.293c-.189.189-.293.439-.293.707s.104.518.293.707c.378.379 1.037.378 1.414.001l5.708-5.708-5.708-5.707c-.189-.189-.439-.293-.707-.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiChevronRightOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiClipboard")]
        TiIcon::TiClipboard => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 3h-10c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-12c0-1.654-1.346-3-3-3zm-8 2h6v1c0 .551-.449 1-1 1h-4c-.551 0-1-.449-1-1v-1zm9 13c0 .551-.449 1-1 1h-10c-.551 0-1-.449-1-1v-12c0-.551.449-1 1-1h1v1c0 1.1.9 2 2 2h4c1.1 0 2-.9 2-2v-1h1c.551 0 1 .449 1 1v12zM16 17h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 14h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 11h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiClipboard".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCloudStorage")]
        TiIcon::TiCloudStorage => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 9l-.351.015c-.825-2.377-3.062-4.015-5.649-4.015-3.309 0-6 2.691-6 6l.001.126c-1.724.445-3.001 2.013-3.001 3.874 0 2.206 1.794 4 4 4h5v-4.586l-1.293 1.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l2.999-2.999c.093-.093.203-.166.326-.217.244-.101.52-.101.764 0 .123.051.233.124.326.217l2.999 2.999c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-1.293-1.293v4.586h4c2.757 0 5-2.243 5-5s-2.243-5-5-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCloudStorage".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCloudStorageOutline")]
        TiIcon::TiCloudStorageOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.5 8.999l-.352.015c-.824-2.375-3.312-4.015-5.898-4.015-3.309 0-6.25 2.69-6.25 6v.126c-1 .445-2.75 2.014-2.75 3.875 0 2.206 2.044 4 4.25 4h11c2.757 0 5-2.244 5-5s-2.243-5.001-5-5.001zm0 8.001h-4.5v-3.794l2.146 2.146c.098.099.226.146.354.146s.256-.049.354-.146c.195-.194.195-.512 0-.707l-2.998-3-.164-.107c-.123-.051-.26-.051-.383 0l-.162.107-3 3c-.194.195-.194.513 0 .707.099.099.227.146.354.146s.256-.049.354-.146l2.145-2.146v3.794h-5.5c-1.104 0-2-.896-2-2s.896-2 1.908-2.005l1.422.015-.248-1.201c-.055-.264-.082-.536-.082-.809 0-2.206 1.794-4 4-4 1.951 0 3.604 1.402 3.93 3.334l.187 1.102 1.073-.306c.312-.089.569-.13.812-.13 1.653 0 3 1.346 3 3s-1.348 3-3.002 3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiCloudStorageOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCode")]
        TiIcon::TiCode => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8.171 18c-.512 0-1.023-.195-1.414-.586l-4.414-4.414 4.414-4.414c.781-.781 2.049-.781 2.828 0 .781.781.781 2.047 0 2.828l-1.585 1.586 1.585 1.586c.781.781.781 2.047 0 2.828-.39.391-.902.586-1.414.586zM15.829 18c-.512 0-1.024-.195-1.414-.586-.781-.781-.781-2.047 0-2.828l1.585-1.586-1.585-1.586c-.781-.781-.781-2.047 0-2.828.779-.781 2.047-.781 2.828 0l4.414 4.414-4.414 4.414c-.39.391-.902.586-1.414.586z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCode".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCodeOutline")]
        TiIcon::TiCodeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M7.828 19c-.801 0-1.555-.312-2.121-.879l-5.121-5.121 5.121-5.121c1.133-1.134 3.112-1.134 4.243.001 1.169 1.168 1.169 3.072-.001 4.241l-.878.879.878.879c1.17 1.169 1.17 3.073 0 4.242-.564.567-1.318.879-2.121.879zm-4.414-6l3.707 3.707c.38.379 1.039.377 1.413.001.391-.391.391-1.025.001-1.415l-2.292-2.293 2.292-2.293c.39-.39.39-1.024 0-1.414-.378-.379-1.036-.377-1.414 0l-3.707 3.707zM16.172 19c-.803 0-1.557-.312-2.122-.88-1.169-1.168-1.169-3.072.001-4.241l.878-.879-.878-.879c-1.17-1.169-1.17-3.073 0-4.242 1.129-1.133 3.109-1.134 4.242 0l5.121 5.121-5.121 5.121c-.566.567-1.32.879-2.121.879zm-.001-10c-.267 0-.518.104-.705.292-.391.391-.391 1.025-.001 1.415l2.292 2.293-2.292 2.293c-.39.39-.39 1.024 0 1.414.377.378 1.035.379 1.414 0l3.707-3.707-3.707-3.707c-.19-.189-.441-.293-.708-.293z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCodeOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCoffee")]
        TiIcon::TiCoffee => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 19h-12c-.553 0-1-.447-1-1s.447-1 1-1h12c.553 0 1 .447 1 1s-.447 1-1 1zM17.5 5h-12.5v9c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2v-2h.5c1.93 0 3.5-1.57 3.5-3.5s-1.57-3.5-3.5-3.5zm-2.5 9h-8v-7h8v7zm2.5-4h-1.5v-3h1.5c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCoffee".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCog")]
        TiIcon::TiCog => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9.387 17.548l.371 1.482c.133.533.692.97 1.242.97h1c.55 0 1.109-.437 1.242-.97l.371-1.482c.133-.533.675-.846 1.203-.694l1.467.42c.529.151 1.188-.114 1.462-.591l.5-.867c.274-.477.177-1.179-.219-1.562l-1.098-1.061c-.396-.383-.396-1.008.001-1.39l1.096-1.061c.396-.382.494-1.084.22-1.561l-.501-.867c-.275-.477-.933-.742-1.461-.591l-1.467.42c-.529.151-1.07-.161-1.204-.694l-.37-1.48c-.133-.532-.692-.969-1.242-.969h-1c-.55 0-1.109.437-1.242.97l-.37 1.48c-.134.533-.675.846-1.204.695l-1.467-.42c-.529-.152-1.188.114-1.462.59l-.5.867c-.274.477-.177 1.179.22 1.562l1.096 1.059c.395.383.395 1.008 0 1.391l-1.098 1.061c-.395.383-.494 1.085-.219 1.562l.501.867c.274.477.933.742 1.462.591l1.467-.42c.528-.153 1.07.16 1.203.693zm2.113-7.048c1.104 0 2 .895 2 2 0 1.104-.896 2-2 2s-2-.896-2-2c0-1.105.896-2 2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCog".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCogOutline")]
        TiIcon::TiCogOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 5l.855 3.42 3.389-.971 1.501 2.6-2.535 2.449 2.535 2.451-1.5 2.6-3.39-.971-.855 3.422h-3l-.855-3.422-3.39.971-1.501-2.6 2.535-2.451-2.534-2.449 1.5-2.6 3.39.971.855-3.42h3m0-2h-3c-.918 0-1.718.625-1.939 1.516l-.354 1.412-1.4-.4c-.184-.053-.369-.078-.552-.078-.7 0-1.368.37-1.731 1l-1.5 2.6c-.459.796-.317 1.802.342 2.438l1.047 1.011-1.048 1.015c-.66.637-.802 1.643-.343 2.438l1.502 2.6c.363.631 1.031 1 1.731 1 .183 0 .368-.025.552-.076l1.399-.401.354 1.415c.222.885 1.022 1.51 1.94 1.51h3c.918 0 1.718-.625 1.939-1.516l.354-1.414 1.399.4c.184.053.369.077.552.077.7 0 1.368-.37 1.731-1l1.5-2.6c.459-.796.317-1.8-.342-2.438l-1.047-1.013 1.047-1.013c.66-.637.801-1.644.342-2.438l-1.5-2.6c-.365-.631-1.031-1-1.732-1-.184 0-.368.025-.551.076l-1.4.401-.354-1.413c-.22-.884-1.02-1.509-1.938-1.509zM11.5 10.5c1.104 0 2 .895 2 2 0 1.104-.896 2-2 2s-2-.896-2-2c0-1.105.896-2 2-2m0-1c-1.654 0-3 1.346-3 3s1.346 3 3 3 3-1.346 3-3-1.346-3-3-3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCogOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCompass")]
        TiIcon::TiCompass => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 5c3.859.001 7 3.142 7 7.001 0 3.858-3.141 6.998-7 6.999-3.859 0-7-3.14-7-6.999s3.141-7 7-7.001m0-2c-4.971.001-9 4.03-9 9.001 0 4.97 4.029 8.999 9 8.999 4.97-.001 9-4.03 9-8.999 0-4.971-4.029-9-9-9.001zM16.182 7.819c-.129-.128-.315-.178-.491-.127l-5.951 1.706c-.166.048-.295.177-.342.343l-1.707 5.951c-.051.175-.002.363.127.491.095.095.223.146.354.146l.138-.02 5.95-1.708c.165-.047.295-.177.342-.343l1.707-5.949c.05-.173.002-.361-.127-.49zm-7.282 7.282l1.383-4.817 3.434 3.435-4.817 1.382z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCompass".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiContacts")]
        TiIcon::TiContacts => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M19 3h-11c-1.654 0-3 1.346-3 3v1h-1c-.553 0-1 .448-1 1s.447 1 1 1h1v2h-1c-.553 0-1 .448-1 1s.447 1 1 1h1v2h-1c-.553 0-1 .448-1 1s.447 1 1 1h1v1c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-12c0-1.654-1.346-3-3-3zm-12 3c0-.551.449-1 1-1v2h-1v-1zm0 3h1v2h-1v-2zm0 4h1v2h-1v-2zm0 5v-1h1v2c-.551 0-1-.449-1-1zm13 0c0 .551-.449 1-1 1h-10v-14h10c.551 0 1 .449 1 1v12z\" />\n  <circle cx=\"14\" cy=\"10.5\" r=\"2\" />\n  <path d=\"M14 13.356c-1.562 0-2.5.715-2.5 1.429 0 .357.938.715 2.5.715 1.466 0 2.5-.357 2.5-.715 0-.714-.98-1.429-2.5-1.429z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiContacts".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCreditCard")]
        TiIcon::TiCreditCard => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 7h-11c-1.654 0-3 1.346-3 3v7c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-7c0-1.654-1.346-3-3-3zm1 10c0 .552-.448 1-1 1h-11c-.552 0-1-.448-1-1v-4h13v4zm0-6h-13v-1c0-.552.448-1 1-1h11c.552 0 1 .448 1 1v1zM14 16h2c.276 0 .5-.224.5-.5s-.224-.5-.5-.5h-2c-.276 0-.5.224-.5.5s.224.5.5.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCreditCard".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiCss3")]
        TiIcon::TiCss3 => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M5.7 3.4l-.6 3.2h12.3l-.4 2.1h-12.3l-.6 3.2h12.3l-.7 3.6-5 1.7-4.3-1.7.3-1.6h-3l-.7 3.8 7.1 2.9 8.2-2.9 1.1-5.8.2-1.2 1.4-7.3h-15.3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiCss3".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDatabase")]
        TiIcon::TiDatabase => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.983 8.791c-.176-3.704-3.236-6.666-6.983-6.666s-6.807 2.962-6.983 6.666l-.017.084v6.25c0 3.86 3.141 7 7 7s7-3.14 7-7v-6.25l-.017-.084zm-6.983 8.834c-2.22 0-4.132-1.324-5-3.222v-.388c1.271 1.3 3.042 2.11 5 2.11s3.729-.811 5-2.11v.388c-.868 1.898-2.78 3.222-5 3.222zm0-13.5c2.757 0 5 2.243 5 5s-2.243 5-5 5-5-2.243-5-5 2.243-5 5-5zm0 16c-2.271 0-4.172-1.532-4.778-3.609 1.188 1.293 2.888 2.109 4.778 2.109s3.59-.816 4.778-2.109c-.606 2.077-2.507 3.609-4.778 3.609z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDatabase".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDelete")]
        TiIcon::TiDelete => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 4c-4.419 0-8 3.582-8 8s3.581 8 8 8 8-3.582 8-8-3.581-8-8-8zm3.707 10.293c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-2.293-2.293-2.293 2.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l2.293-2.293-2.293-2.293c-.391-.391-.391-1.023 0-1.414s1.023-.391 1.414 0l2.293 2.293 2.293-2.293c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414l-2.293 2.293 2.293 2.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDelete".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDeleteOutline")]
        TiIcon::TiDeleteOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 3c-4.963 0-9 4.038-9 9s4.037 9 9 9 9-4.038 9-9-4.037-9-9-9zm0 16c-3.859 0-7-3.14-7-7s3.141-7 7-7 7 3.14 7 7-3.141 7-7 7zM12.707 12l2.646-2.646c.194-.194.194-.512 0-.707-.195-.194-.513-.194-.707 0l-2.646 2.646-2.646-2.647c-.195-.194-.513-.194-.707 0-.195.195-.195.513 0 .707l2.646 2.647-2.646 2.646c-.195.195-.195.513 0 .707.097.098.225.147.353.147s.256-.049.354-.146l2.646-2.647 2.646 2.646c.098.098.226.147.354.147s.256-.049.354-.146c.194-.194.194-.512 0-.707l-2.647-2.647z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDeleteOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDeviceDesktop")]
        TiIcon::TiDeviceDesktop => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M21 1h-18c-1.654 0-3 1.346-3 3v11c0 1.654 1.346 3 3 3h6v2h-3c-.552 0-1 .448-1 1s.448 1 1 1h12c.552 0 1-.448 1-1s-.448-1-1-1h-3v-2h6c1.654 0 3-1.346 3-3v-11c0-1.654-1.346-3-3-3zm-7 19h-4v-2h4v2zm8-5c0 .551-.449 1-1 1h-18c-.551 0-1-.449-1-1v-11c0-.551.449-1 1-1h18c.551 0 1 .449 1 1v11zM20 4h-16c-.55 0-1 .45-1 1v8c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-8c0-.55-.45-1-1-1zm0 9h-16v-8h16v8z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDeviceDesktop".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDeviceLaptop")]
        TiIcon::TiDeviceLaptop => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.989 16.049c.009-.315.011-.657.011-1.049v-9c0-1.654-1.346-3-3-3h-14c-1.654 0-3 1.346-3 3v9c0 .385.002.73.012 1.049-1.145.228-2.012 1.24-2.012 2.451 0 1.378 1.122 2.5 2.5 2.5h19c1.378 0 2.5-1.122 2.5-2.5 0-1.211-.866-2.222-2.011-2.451zm-17.989-10.049c0-.551.449-1 1-1h14c.551 0 1 .449 1 1v9c0 .388-.005.726-.014 1h-.986v-9c0-.55-.45-1-1-1h-12c-.55 0-1 .45-1 1v9h-.98c-.012-.264-.02-.599-.02-1v-9zm14 10h-12v-9h12v9zm3.5 3h-19c-.271 0-.5-.229-.5-.5s.229-.5.5-.5h19c.271 0 .5.229.5.5s-.229.5-.5.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDeviceLaptop".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDevicePhone")]
        TiIcon::TiDevicePhone => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M15 3h-7c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h7c1.654 0 3-1.346 3-3v-12c0-1.654-1.346-3-3-3zm1 15c0 .551-.449 1-1 1h-7c-.551 0-1-.449-1-1v-12c0-.551.449-1 1-1h7c.551 0 1 .449 1 1v12zM14 6h-5c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h1.5c0 .553.448 1 1 1s1-.447 1-1h1.5c.55 0 1-.45 1-1v-9c0-.55-.45-1-1-1zm0 10h-5v-9h5v9z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDevicePhone".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDeviceTablet")]
        TiIcon::TiDeviceTablet => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 4h-9c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h3.5c0 .553.448 1 1 1s1-.447 1-1h3.5c.55 0 1-.45 1-1v-12c0-.55-.45-1-1-1zm0 13h-9v-12h9v12zM18 1h-11c-1.654 0-3 1.346-3 3v15c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-15c0-1.654-1.346-3-3-3zm1 18c0 .551-.449 1-1 1h-11c-.551 0-1-.449-1-1v-15c0-.551.449-1 1-1h11c.551 0 1 .449 1 1v15z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDeviceTablet".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDirections")]
        TiIcon::TiDirections => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.908 9.5l-2.754-2.607c-.568-.541-1.447-.893-2.237-.893h-2.917v-.5c0-.828-.672-1.5-1.5-1.5s-1.5.672-1.5 1.5v.5h-3.5c-1.93 0-3.5 1.57-3.5 3.5 0 1.396.828 2.596 2.016 3.157l-1.844 1.843 2.561 2.561c.57.57 1.46.939 2.268.939h2.2l.8 4h1l.8-4h2.7c1.931 0 3.5-1.57 3.5-3.5 0-.902-.353-1.718-.915-2.339l.072-.056 2.75-2.605zm-5.408 6.5h-7.5c-.275 0-.658-.158-.854-.354l-1.146-1.146 1.146-1.146c.195-.195.577-.354.854-.354h7.5c.828 0 1.5.672 1.5 1.5s-.672 1.5-1.5 1.5zm1.279-5.344c-.199.19-.586.344-.862.344h-9.417c-.828 0-1.5-.672-1.5-1.5s.672-1.5 1.5-1.5h9.417c.276 0 .663.154.862.344l1.221 1.156-1.221 1.156z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDirections".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDivide")]
        TiIcon::TiDivide => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<circle cx=\"12\" cy=\"6\" r=\"2.25\" />\n<circle cx=\"12\" cy=\"18\" r=\"2.25\" />\n<path d=\"M18 10h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12c1.104 0 2-.896 2-2s-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDivide".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDivideOutline")]
        TiIcon::TiDivideOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 8.5c-1.654 0-3-1.346-3-3s1.346-3 3-3 3 1.346 3 3-1.346 3-3 3zm0-4c-.552 0-1 .449-1 1s.448 1 1 1 1-.449 1-1-.448-1-1-1zM12 21.5c-1.654 0-3-1.346-3-3s1.346-3 3-3 3 1.346 3 3-1.346 3-3 3zm0-4c-.552 0-1 .449-1 1s.448 1 1 1 1-.449 1-1-.448-1-1-1zM18 15h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.552 0-1 .449-1 1s.448 1 1 1h12c.552 0 1-.449 1-1s-.448-1-1-1h-12z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDivideOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDocument")]
        TiIcon::TiDocument => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.707 7.293l-4-4c-.187-.188-.441-.293-.707-.293h-8c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-10c0-.266-.105-.52-.293-.707zm-2.121.707h-1.086c-.827 0-1.5-.673-1.5-1.5v-1.086l2.586 2.586zm-.586 11h-10c-.552 0-1-.448-1-1v-12c0-.552.448-1 1-1h7v1.5c0 1.379 1.121 2.5 2.5 2.5h1.5v9c0 .552-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDocument".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDocumentAdd")]
        TiIcon::TiDocumentAdd => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15 12h-2v-2c0-.553-.447-1-1-1s-1 .447-1 1v2h-2c-.553 0-1 .447-1 1s.447 1 1 1h2v2c0 .553.447 1 1 1s1-.447 1-1v-2h2c.553 0 1-.447 1-1s-.447-1-1-1zM19.707 7.293l-4-4c-.187-.188-.441-.293-.707-.293h-8c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-10c0-.266-.105-.52-.293-.707zm-2.121.707h-1.086c-.827 0-1.5-.673-1.5-1.5v-1.086l2.586 2.586zm-.586 11h-10c-.552 0-1-.448-1-1v-12c0-.552.448-1 1-1h7v1.5c0 1.379 1.121 2.5 2.5 2.5h1.5v9c0 .552-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDocumentAdd".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDocumentDelete")]
        TiIcon::TiDocumentDelete => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.707 7.293l-4-4c-.187-.188-.441-.293-.707-.293h-8c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-10c0-.266-.105-.52-.293-.707zm-2.121.707h-1.086c-.827 0-1.5-.673-1.5-1.5v-1.086l2.586 2.586zm-.586 11h-10c-.552 0-1-.448-1-1v-12c0-.552.448-1 1-1h7v1.5c0 1.379 1.121 2.5 2.5 2.5h1.5v9c0 .552-.448 1-1 1zM15 14h-6c-.553 0-1-.447-1-1s.447-1 1-1h6c.553 0 1 .447 1 1s-.447 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiDocumentDelete".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDocumentText")]
        TiIcon::TiDocumentText => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 21h-10c-1.654 0-3-1.346-3-3v-12c0-1.654 1.346-3 3-3h10c1.654 0 3 1.346 3 3v12c0 1.654-1.346 3-3 3zm-10-16c-.551 0-1 .449-1 1v12c0 .551.449 1 1 1h10c.551 0 1-.449 1-1v-12c0-.551-.449-1-1-1h-10zM16 11h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 8h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 14h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5zM16 17h-8c-.276 0-.5-.224-.5-.5s.224-.5.5-.5h8c.276 0 .5.224.5.5s-.224.5-.5.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDocumentText".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDownload")]
        TiIcon::TiDownload => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.707 7.404c-.189-.188-.448-.283-.707-.283s-.518.095-.707.283l-2.293 2.293v-6.697c0-.552-.448-1-1-1s-1 .448-1 1v6.697l-2.293-2.293c-.189-.188-.44-.293-.707-.293s-.518.105-.707.293c-.39.39-.39 1.024 0 1.414l4.707 4.682 4.709-4.684c.388-.387.388-1.022-.002-1.412zM20.987 16c0-.105-.004-.211-.039-.316l-2-6c-.136-.409-.517-.684-.948-.684h-.219c-.094.188-.21.368-.367.525l-1.482 1.475h1.348l1.667 5h-13.893l1.667-5h1.348l-1.483-1.475c-.157-.157-.274-.337-.367-.525h-.219c-.431 0-.812.275-.948.684l-2 6c-.035.105-.039.211-.039.316-.013 0-.013 5-.013 5 0 .553.447 1 1 1h16c.553 0 1-.447 1-1 0 0 0-5-.013-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDownload".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDownloadOutline")]
        TiIcon::TiDownloadOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.986 17c0-.105-.004-.211-.038-.316l-2-6c-.136-.409-.516-.684-.948-.684h-.561l.682-.678c1.17-1.17 1.17-3.072 0-4.242-.81-.812-2.068-1.078-3.121-.709v-1.371c0-1.654-1.346-3-3-3s-3 1.346-3 3v1.371c-1.052-.369-2.311-.103-3.121.709-1.17 1.17-1.17 3.072.002 4.244l.68.676h-.561c-.432 0-.812.275-.948.684l-2 6c-.034.105-.038.211-.038.316-.014 0-.014 5-.014 5 0 .553.447 1 1 1h16c.553 0 1-.447 1-1 0 0 0-5-.014-5zm-13.693-10.506c.189-.187.439-.293.707-.293s.518.104.707.293l2.293 2.293v-5.787c0-.552.448-1 1-1s1 .448 1 1v5.787l2.293-2.293c.379-.377 1.035-.377 1.414 0 .391.39.391 1.023.002 1.412l-4.709 4.684-4.707-4.682c-.391-.388-.391-1.024 0-1.414zm-.572 5.506h1.852l3.429 3.41 3.428-3.41h1.852l1.667 5h-13.894l1.666-5zm12.279 9h-14v-3h14v3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiDownloadOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiDropbox")]
        TiIcon::TiDropbox => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M3 12.9l5.3 3.5 3.7-3.1-5.3-3.3zM8.3 3.6l-5.3 3.5 3.7 2.9 5.3-3.3zM21 7.1l-5.3-3.5-3.7 3.1 5.3 3.3zM12 13.3l3.7 3.1 5.3-3.5-3.7-2.9zM12 14.5l-3.7 3.1-1.6-1.1v1.2l5.3 3.2 5.3-3.2v-1.2l-1.6 1.1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiDropbox".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEdit")]
        TiIcon::TiEdit => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.561 5.318l-2.879-2.879c-.293-.293-.677-.439-1.061-.439-.385 0-.768.146-1.061.439l-3.56 3.561h-9c-.552 0-1 .447-1 1v13c0 .553.448 1 1 1h13c.552 0 1-.447 1-1v-9l3.561-3.561c.293-.293.439-.677.439-1.061s-.146-.767-.439-1.06zm-10.061 9.354l-2.172-2.172 6.293-6.293 2.172 2.172-6.293 6.293zm-2.561-1.339l1.756 1.728-1.695-.061-.061-1.667zm7.061 5.667h-11v-11h6l-3.18 3.18c-.293.293-.478.812-.629 1.289-.16.5-.191 1.056-.191 1.47v3.061h3.061c.414 0 1.108-.1 1.571-.29.464-.19.896-.347 1.188-.64l3.18-3.07v6zm2.5-11.328l-2.172-2.172 1.293-1.293 2.171 2.172-1.292 1.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEdit".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEject")]
        TiIcon::TiEject => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 17.5c-2.481 0-4.5-2.019-4.5-4.5 0-.553-.447-1-1-1s-1 .447-1 1c0 3.584 2.916 6.5 6.5 6.5s6.5-2.916 6.5-6.5-2.916-6.5-6.5-6.5c-.553 0-1 .447-1 1s.447 1 1 1c2.481 0 4.5 2.019 4.5 4.5s-2.019 4.5-4.5 4.5zM10.656 4c.552 0 1 .448 1 1s-.448 1-1 1h-3.243l1.708 1.707 4.093 4.092c.391.391.391 1.025.001 1.415-.189.188-.44.292-.708.292s-.519-.104-.707-.291l-4.093-4.094-1.707-1.708v3.243c0 .552-.448 1-1 1s-1-.448-1-1v-6.656h6.656\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEject".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEjectOutline")]
        TiIcon::TiEjectOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16 8.551v-1.551c0-1.105-.896-2-2-2h-10v10c0 1.104.896 2 2 2h1.066c.893 2.887 3.646 5 6.809 5 3.859 0 7.062-3.016 7.062-6.875.001-3.161-2.068-5.74-4.937-6.574zm-8 1.862v3.243c0 .552-.448 1-1 1s-1-.448-1-1v-6.656h6.656c.552 0 1 .448 1 1s-.448 1-1 1h-3.243l4.801 4.799c.392.391.392 1.025.001 1.415-.189.188-.439.292-.708.292-.268 0-.519-.104-.707-.291l-4.8-4.802zm6 9.618c-2.757 0-5-2.26-5-5.016 0-.705-.004-1.371.21-1.979l2.883 2.884c.39.39.901.584 1.414.584s1.022-.194 1.414-.584c.781-.78.781-2.049 0-2.83l-2.567-2.567c.517-.226 1.065-.398 1.646-.398 2.757 0 5 2.182 5 4.938 0 2.757-2.243 4.968-5 4.968z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEjectOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEquals")]
        TiIcon::TiEquals => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 7h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12c1.104 0 2-.896 2-2s-.896-2-2-2zM18 14h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12c1.104 0 2-.896 2-2s-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEquals".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEqualsOutline")]
        TiIcon::TiEqualsOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 12h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.552 0-1 .449-1 1s.448 1 1 1h12c.552 0 1-.449 1-1s-.448-1-1-1h-12zM18 19h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.552 0-1 .449-1 1s.448 1 1 1h12c.552 0 1-.449 1-1s-.448-1-1-1h-12z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEqualsOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiExport")]
        TiIcon::TiExport => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 16.5v.5c1.691-2.578 3.6-3.953 6-4v3c0 .551.511 1 1.143 1 .364 0 .675-.158.883-.391 1.933-2.029 5.974-6.109 5.974-6.109s-4.041-4.082-5.975-6.137c-.208-.205-.518-.363-.882-.363-.632 0-1.143.447-1.143 1v3c-4.66 0-6 4.871-6 8.5zM5 21h14c.553 0 1-.448 1-1v-6.046c-.664.676-1.364 1.393-2 2.047v2.999h-12v-12h7v-2h-8c-.553 0-1 .448-1 1v14c0 .552.447 1 1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiExport".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiExportOutline")]
        TiIcon::TiExportOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22.711 9.796c-.041-.041-4.055-4.096-5.982-6.146-.42-.414-.999-.65-1.586-.65-1.182 0-2.143.896-2.143 2h-8c-.553 0-1 .448-1 1v14c0 .552.447 1 1 1h14c.553 0 1-.448 1-1v-6.045c1.434-1.461 2.688-2.729 2.711-2.751.387-.39.387-1.018 0-1.408zm-7.432 6.145l-.136.059-.144-.04v-3.96h-1c-1.771.034-3.336.68-4.753 1.958.43-2.215 1.6-4.958 4.753-4.958h1v-3.958l.144-.042.154.05c1.436 1.525 4.051 4.187 5.297 5.45-.253.257-4.342 4.422-5.315 5.441zm-9.279 3.059v-12h8v1c-4.66 0-6 4.871-6 8.5v.5c1.691-2.578 3.6-3.953 6-4v3c0 .551.512 1 1.143 1 .364 0 .676-.158.883-.391.539-.565 1.242-1.291 1.976-2.043v4.434h-12.002z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiExportOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEye")]
        TiIcon::TiEye => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.821 12.43c-.083-.119-2.062-2.944-4.793-4.875-1.416-1.003-3.202-1.555-5.028-1.555-1.825 0-3.611.552-5.03 1.555-2.731 1.931-4.708 4.756-4.791 4.875-.238.343-.238.798 0 1.141.083.119 2.06 2.944 4.791 4.875 1.419 1.002 3.205 1.554 5.03 1.554 1.826 0 3.612-.552 5.028-1.555 2.731-1.931 4.71-4.756 4.793-4.875.239-.342.239-.798 0-1.14zm-9.821 4.07c-1.934 0-3.5-1.57-3.5-3.5 0-1.934 1.566-3.5 3.5-3.5 1.93 0 3.5 1.566 3.5 3.5 0 1.93-1.57 3.5-3.5 3.5zM14 13c0 1.102-.898 2-2 2-1.105 0-2-.898-2-2 0-1.105.895-2 2-2 1.102 0 2 .895 2 2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEye".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiEyeOutline")]
        TiIcon::TiEyeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 9c1.211 0 2.381.355 3.297 1.004 1.301.92 2.43 2.124 3.165 2.996-.735.872-1.864 2.077-3.166 2.996-.915.649-2.085 1.004-3.296 1.004s-2.382-.355-3.299-1.004c-1.301-.92-2.43-2.124-3.164-2.996.734-.872 1.863-2.076 3.164-2.995.917-.65 2.088-1.005 3.299-1.005m0-2c-1.691 0-3.242.516-4.453 1.371-2.619 1.852-4.547 4.629-4.547 4.629s1.928 2.777 4.547 4.629c1.211.855 2.762 1.371 4.453 1.371s3.242-.516 4.451-1.371c2.619-1.852 4.549-4.629 4.549-4.629s-1.93-2.777-4.549-4.629c-1.209-.855-2.76-1.371-4.451-1.371zM12 12c-.553 0-1 .447-1 1 0 .551.447 1 1 1 .551 0 1-.449 1-1 0-.553-.449-1-1-1zM12 16c-1.654 0-3-1.346-3-3s1.346-3 3-3 3 1.346 3 3-1.346 3-3 3zm0-5c-1.104 0-2 .896-2 2s.896 2 2 2 2-.896 2-2-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiEyeOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFeather")]
        TiIcon::TiFeather => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.68 1.017l-.18-.034-.18.033c-4.821.884-8.32 5.084-8.32 9.984 0 4.617 3.108 8.61 7.5 9.795v1.205c0 .553.448 1 1 1s1-.447 1-1v-1.205c4.392-1.185 7.5-5.178 7.5-9.795 0-4.9-3.499-9.1-8.32-9.983zm.82 17.683v-11.7c0-.553-.448-1-1-1s-1 .447-1 1v11.7c-3.18-1.093-5.4-4.054-5.49-7.483l3.137 3.137c.097.097.225.146.353.146s.256-.049.354-.146c.195-.195.195-.512 0-.707l-3.769-3.769c.133-.964.434-1.877.877-2.709l2.184 2.185c.098.097.226.146.354.146s.256-.049.354-.146c.195-.195.195-.512 0-.707l-2.353-2.353c1.162-1.641 2.919-2.846 4.999-3.275 2.08.43 3.837 1.635 4.999 3.275l-2.353 2.353c-.195.195-.195.512 0 .707.098.097.226.146.354.146s.256-.049.354-.146l2.184-2.185c.444.832.744 1.745.877 2.709l-3.769 3.769c-.195.195-.195.512 0 .707.098.098.226.146.354.146s.256-.049.354-.146l3.137-3.137c-.091 3.429-2.311 6.39-5.491 7.483z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFeather".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFilm")]
        TiIcon::TiFilm => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M8 8v7h8v-7h-8zm7 6h-6v-5h6v5zM17 2h-3v2h-4v-2h-3c-1.654 0-3 1.346-3 3v13c0 1.654 1.346 3 3 3h3v-2h4v2h3c1.654 0 3-1.346 3-3v-13c0-1.654-1.346-3-3-3zm1 4c-.553 0-1 .447-1 1s.447 1 1 1v1c-.553 0-1 .447-1 1s.447 1 1 1v1c-.553 0-1 .447-1 1s.447 1 1 1v1c-.553 0-1 .447-1 1s.447 1 1 1v1c0 .551-.448 1-1 1h-1v-2h-8v2h-1c-.552 0-1-.449-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c.553 0 1-.447 1-1s-.447-1-1-1v-1c0-.551.448-1 1-1h1v2h8v-2h1c.552 0 1 .449 1 1v1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFilm".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFilter")]
        TiIcon::TiFilter => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 6h-14c-1.1 0-1.4.6-.6 1.4l4.2 4.2c.8.8 1.4 2.3 1.4 3.4v5l4-2v-3.5c0-.8.6-2.1 1.4-2.9l4.2-4.2c.8-.8.5-1.4-.6-1.4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFilter".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlag")]
        TiIcon::TiFlag => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.383 4.318c-.374-.155-.804-.069-1.09.217-1.264 1.263-3.321 1.264-4.586 0-2.045-2.043-5.37-2.043-7.414 0-.188.187-.293.442-.293.707v13c0 .552.447 1 1 1s1-.448 1-1v-4.553c1.271-.997 3.121-.911 4.293.26 2.045 2.043 5.371 2.043 7.414 0 .188-.188.293-.442.293-.707v-8c0-.405-.244-.769-.617-.924z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlag".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlagOutline")]
        TiIcon::TiFlagOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.383 4.318c-.374-.155-.804-.069-1.09.217-1.264 1.263-3.321 1.264-4.586 0-2.045-2.043-5.37-2.043-7.414 0-.188.187-.293.442-.293.707v13c0 .552.448 1 1 1s1-.448 1-1v-4.553c1.271-.997 3.121-.911 4.293.26 2.045 2.043 5.371 2.043 7.414 0 .188-.188.293-.442.293-.707v-8c0-.405-.244-.769-.617-.924zm-7.09 1.631c1.54 1.539 3.808 1.918 5.707 1.138v2.311c-1.446.916-3.387.749-4.646-.51-1.448-1.447-3.598-1.743-5.354-.927v-2.272c1.271-.997 3.121-.912 4.293.26zm1.414 6.585c-1.022-1.021-2.365-1.532-3.707-1.532-.681 0-1.361.131-2 .394v-2.311c1.446-.916 3.387-.749 4.646.51.925.924 2.139 1.386 3.354 1.386.687 0 1.366-.164 2-.459v2.272c-1.272.997-3.122.911-4.293-.26z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlagOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlash")]
        TiIcon::TiFlash => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.502 12.033l-4.241-2.458 2.138-5.131c.066-.134.103-.285.103-.444 0-.552-.445-1-.997-1-.249.004-.457.083-.622.214l-.07.06-7.5 7.1c-.229.217-.342.529-.306.842.036.313.219.591.491.75l4.242 2.46-2.163 5.19c-.183.436-.034.94.354 1.208.173.118.372.176.569.176.248 0 .496-.093.688-.274l7.5-7.102c.229-.217.342-.529.306-.842-.037-.313-.22-.591-.492-.749z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlash".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlashOutline")]
        TiIcon::TiFlashOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14.5 4h.005m-.005 0l-2.5 6 5 2.898-7.5 7.102 2.5-6-5-2.9 7.5-7.1m0-2c-.562.012-1.029.219-1.379.551l-7.497 7.095c-.458.435-.685 1.059-.61 1.686.072.626.437 1.182.982 1.498l3.482 2.021-1.826 4.381c-.362.871-.066 1.879.712 2.416.344.236.739.354 1.135.354.498 0 .993-.186 1.375-.548l7.5-7.103c.458-.434.685-1.058.61-1.685-.073-.627-.438-1.183-.982-1.498l-3.482-2.018 1.789-4.293c.123-.26.192-.551.192-.857 0-1.102-.89-1.996-2.001-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlashOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlowChildren")]
        TiIcon::TiFlowChildren => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 16c-1.305 0-2.403.837-2.816 2h-3.184c-1.654 0-3-1.346-3-3v-3.025c.838.634 1.87 1.025 3 1.025h3.184c.413 1.163 1.512 2 2.816 2 1.657 0 3-1.343 3-3s-1.343-3-3-3c-1.305 0-2.403.837-2.816 2h-3.184c-1.654 0-3-1.346-3-3v-.184c1.163-.413 2-1.512 2-2.816 0-1.657-1.343-3-3-3s-3 1.343-3 3c0 1.304.837 2.403 2 2.816v7.184c0 2.757 2.243 5 5 5h3.184c.413 1.163 1.512 2 2.816 2 1.657 0 3-1.343 3-3s-1.343-3-3-3zm0-5c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm-10-7c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm10 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlowChildren".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlowMerge")]
        TiIcon::TiFlowMerge => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 16.184v-1.851c0-1.93-1.57-3.5-3.5-3.5-.827 0-1.5-.673-1.5-1.5v-1.517c1.161-.415 2-1.514 2-2.816 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815v1.518c0 .827-.673 1.5-1.5 1.5-1.93 0-3.5 1.57-3.5 3.5v1.851c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816v-1.851c0-.827.673-1.5 1.5-1.5.979 0 1.864-.407 2.5-1.058.636.651 1.521 1.058 2.5 1.058.827 0 1.5.673 1.5 1.5v1.851c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816zm-11 3.816c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1zm5-16c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm5 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlowMerge".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlowParallel")]
        TiIcon::TiFlowParallel => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M18 16.184v-8.368c1.161-.415 2-1.514 2-2.816 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815v8.369c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816zm-1-12.184c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm0 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1zM10 5c0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815v8.369c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816v-8.368c1.161-.415 2-1.514 2-2.816zm-3-1c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm0 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlowParallel".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFlowSwitch")]
        TiIcon::TiFlowSwitch => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M8 16.184v-.684c0-.848.512-1.595 1.287-2.047-.667-.282-1.279-.667-1.822-1.131-.904.814-1.465 1.938-1.465 3.178v.684c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816zm-1 3.816c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1zM16 7.815v.351c0 .985-.535 1.852-1.345 2.36.665.274 1.279.646 1.823 1.1.936-.878 1.522-2.102 1.522-3.459v-.351c1.161-.415 2-1.514 2-2.816 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815zm1-3.815c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zM17.935 16.164c-.41-2.913-2.911-5.164-5.935-5.164-1.936 0-3.552-1.381-3.92-3.209 1.12-.436 1.92-1.519 1.92-2.791 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.326.87 2.44 2.065 2.836.41 2.913 2.911 5.164 5.935 5.164 1.936 0 3.552 1.381 3.92 3.209-1.12.436-1.92 1.519-1.92 2.791 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.326-.87-2.44-2.065-2.836zm-10.935-12.164c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm10 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFlowSwitch".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFolder")]
        TiIcon::TiFolder => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 6h-6c0-1.104-.896-2-2-2h-4c-1.654 0-3 1.346-3 3v10c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-8c0-1.654-1.346-3-3-3zm-12 0h4c0 1.104.896 2 2 2h6c.552 0 1 .448 1 1h-14v-2c0-.552.448-1 1-1zm12 12h-12c-.552 0-1-.448-1-1v-7h14v7c0 .552-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFolder".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFolderAdd")]
        TiIcon::TiFolderAdd => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 6h-6c0-1.104-.896-2-2-2h-4c-1.654 0-3 1.346-3 3v10c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-8c0-1.654-1.346-3-3-3zm0 12h-12c-.552 0-1-.448-1-1v-7h4c.275 0 .5-.225.5-.5s-.225-.5-.5-.5h-4v-2c0-.552.448-1 1-1h4c0 1.104.896 2 2 2h6c.552 0 1 .448 1 1h-4c-.275 0-.5.225-.5.5s.225.5.5.5h4v7c0 .552-.448 1-1 1zM15 12h-2v-2c0-.553-.447-1-1-1s-1 .447-1 1v2h-2c-.553 0-1 .447-1 1s.447 1 1 1h2v2c0 .553.447 1 1 1s1-.447 1-1v-2h2c.553 0 1-.447 1-1s-.447-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFolderAdd".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFolderDelete")]
        TiIcon::TiFolderDelete => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 6h-6c0-1.104-.896-2-2-2h-4c-1.654 0-3 1.346-3 3v10c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-8c0-1.654-1.346-3-3-3zm-12 0h4c0 1.104.896 2 2 2h6c.552 0 1 .448 1 1h-14v-2c0-.552.448-1 1-1zm12 12h-12c-.552 0-1-.448-1-1v-7h14v7c0 .552-.448 1-1 1zM15 14h-6c-.553 0-1-.447-1-1s.447-1 1-1h6c.553 0 1 .447 1 1s-.447 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFolderDelete".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiFolderOpen")]
        TiIcon::TiFolderOpen => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22.3 8h-2.4c-.4-1.2-1.5-2-2.8-2h-6c0-1.1-.9-2-2-2h-4.1c-1.7 0-3 1.3-3 3v10c0 1.7 1.3 3 3 3h12c1.7 0 3.4-1.3 3.8-3l2.2-8c.1-.6-.2-1-.7-1zm-18.3 1v-2c0-.6.4-1 1-1h4c0 1.1.9 2 2 2h6c.6 0 1 .4 1 1h-11.1c-.6 0-1.1.4-1.3 1l-1.6 6.3v-7.3zm14.9 7.5c-.2.8-1.1 1.5-1.9 1.5h-12s-.4-.2-.2-.8l1.9-7c0-.1.2-.2.3-.2h13.7l-1.8 6.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiFolderOpen".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiGift")]
        TiIcon::TiGift => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 8h-2.352c.219-.457.352-.961.352-1.5 0-1.93-1.57-3.5-3.5-3.5-.979 0-1.864.407-2.5 1.058-.636-.651-1.521-1.058-2.5-1.058-1.93 0-3.5 1.57-3.5 3.5 0 .539.133 1.043.352 1.5h-2.352c-.553 0-1 .448-1 1v4c0 .552.447 1 1 1v5c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-5c.553 0 1-.448 1-1v-4c0-.552-.447-1-1-1zm-1 4h-5v-2h5v2zm-8-5h2v1h-2v-1zm2 3v2h-2v-2h2zm1.5-5c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5c-.177 0-.344-.039-.5-.097v-.903c0-.521-.404-.937-.913-.982.202-.59.756-1.018 1.413-1.018zm-6.5 1.5c0-.827.673-1.5 1.5-1.5.657 0 1.211.428 1.413 1.018-.509.045-.913.461-.913.982v.903c-.156.058-.323.097-.5.097-.827 0-1.5-.673-1.5-1.5zm2 3.5v2h-5v-2h5zm-3 10c-.551 0-1-.449-1-1v-6h4v7h-3zm4 0v-7h2v7h-2zm6 0h-3v-7h4v6c0 .551-.449 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiGift".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiGlobe")]
        TiIcon::TiGlobe => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11 20h-4c-.553 0-1 .447-1 1s.447 1 1 1h10c.553 0 1-.447 1-1s-.447-1-1-1h-4v-1.23c1.64-.371 3.146-1.188 4.363-2.406 1.7-1.7 2.637-3.96 2.637-6.364 0-2.067-.692-4.029-1.968-5.619l.675-.673c.391-.391.391-1.023.001-1.415-.391-.391-1.024-.39-1.415-.001l-2.052 2.049.708.708c1.322 1.322 2.051 3.081 2.051 4.951s-.729 3.627-2.051 4.949-3.079 2.051-4.949 2.051-3.627-.729-4.949-2.051c-.391-.391-1.023-.391-1.414 0-.391.39-.391 1.023 0 1.414 1.699 1.7 3.959 2.637 6.363 2.637v1zM11 4c1.657 0 3.157.672 4.243 1.757 1.085 1.086 1.757 2.586 1.757 4.243 0 1.656-.672 3.156-1.757 4.242-1.086 1.086-2.586 1.758-4.243 1.758-1.658 0-3.157-.672-4.242-1.757-1.085-1.086-1.756-2.586-1.756-4.243s.671-3.157 1.756-4.243c1.085-1.085 2.584-1.757 4.242-1.757z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiGlobe".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiGlobeOutline")]
        TiIcon::TiGlobeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11 6c2.206 0 4 1.794 4 4s-1.794 4-4 4c-2.204 0-3.998-1.794-3.998-4s1.794-4 3.998-4m0-2c-3.314 0-5.998 2.686-5.998 6s2.684 6 5.998 6c3.312 0 6-2.688 6-6 0-3.314-2.688-6-6-6zM17 20h-4v-1.23c1.641-.371 3.146-1.188 4.363-2.406 1.699-1.699 2.637-3.96 2.637-6.363 0-2.067-.691-4.028-1.968-5.619l.675-.673c.391-.391.391-1.023.001-1.415-.392-.392-1.024-.39-1.415-.001l-2.052 2.049.708.708c1.322 1.321 2.051 3.08 2.051 4.95s-.729 3.627-2.051 4.949-3.079 2.051-4.949 2.051-3.627-.729-4.949-2.051c-.391-.391-1.023-.391-1.414 0-.391.39-.391 1.023 0 1.414 1.699 1.699 3.959 2.637 6.363 2.637v1h-4c-.553 0-1 .447-1 1s.447 1 1 1h10c.553 0 1-.447 1-1s-.447-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiGlobeOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiGroup")]
        TiIcon::TiGroup => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 14c1.381 0 2.631-.56 3.536-1.465.904-.904 1.464-2.154 1.464-3.535s-.56-2.631-1.464-3.535c-.905-.905-2.155-1.465-3.536-1.465s-2.631.56-3.536 1.465c-.904.904-1.464 2.154-1.464 3.535s.56 2.631 1.464 3.535c.905.905 2.155 1.465 3.536 1.465zM20 15c.69 0 1.315-.279 1.768-.731.453-.452.732-1.077.732-1.769 0-.69-.279-1.315-.732-1.768-.453-.453-1.078-.732-1.768-.732-.691 0-1.316.279-1.769.732-.452.453-.731 1.078-.731 1.768 0 .691.279 1.316.731 1.769s1.078.731 1.769.731zM20 15.59c-1.331 0-2.332.406-2.917.968-1.115-.917-2.878-1.558-5.083-1.558-2.266 0-3.995.648-5.092 1.564-.596-.565-1.608-.974-2.908-.974-2.188 0-3.5 1.09-3.5 2.182 0 .545 1.312 1.092 3.5 1.092.604 0 1.146-.051 1.623-.133l-.04.27c0 1 2.406 2 6.417 2 3.762 0 6.417-1 6.417-2l-.02-.255c.463.073.995.118 1.603.118 2.051 0 3.5-.547 3.5-1.092 0-1.092-1.373-2.182-3.5-2.182zM4 15c.69 0 1.315-.279 1.768-.732.453-.453.732-1.078.732-1.768 0-.689-.279-1.314-.732-1.768-.453-.452-1.078-.732-1.768-.732-.691 0-1.316.28-1.769.732-.452.454-.731 1.079-.731 1.768 0 .69.279 1.315.731 1.768.453.453 1.078.732 1.769.732z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiGroup".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiGroupOutline")]
        TiIcon::TiGroupOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 14c2.764 0 5-2.238 5-5s-2.236-5-5-5-5 2.238-5 5 2.236 5 5 5zm0-8c1.654 0 3 1.346 3 3s-1.346 3-3 3-3-1.346-3-3 1.346-3 3-3zM20 15c1.381 0 2.5-1.117 2.5-2.5 0-1.381-1.119-2.5-2.5-2.5-1.382 0-2.5 1.119-2.5 2.5 0 1.383 1.118 2.5 2.5 2.5zm0-4c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5-1.5-.673-1.5-1.5.673-1.5 1.5-1.5zM20 15.59c-1.331 0-2.332.406-2.917.969-1.115-.918-2.878-1.559-5.083-1.559-2.266 0-3.995.648-5.092 1.564-.596-.565-1.608-.975-2.908-.975-2.188 0-3.5 1.091-3.5 2.183 0 .545 1.312 1.092 3.5 1.092.604 0 1.146-.051 1.623-.133l-.04.27c0 1 2.405 2 6.417 2 3.762 0 6.417-1 6.417-2l-.021-.255c.463.073.996.118 1.604.118 2.051 0 3.5-.547 3.5-1.092 0-1.092-1.373-2.182-3.5-2.182zm-16 2.273c-1.309 0-2.068-.207-2.417-.354.239-.405 1.003-.92 2.417-.92 1.107 0 1.837.351 2.208.706l-.235.344c-.452.119-1.108.224-1.973.224zm8 1.137c-2.163 0-3.501-.312-4.184-.561.521-.678 1.918-1.439 4.184-1.439 2.169 0 3.59.761 4.148 1.425-.755.27-2.162.575-4.148.575zm8-1.137c-.914 0-1.546-.103-1.973-.213-.072-.127-.155-.252-.248-.375.356-.345 1.071-.685 2.221-.685 1.324 0 2.141.501 2.404.911-.39.163-1.205.362-2.404.362zM4 15c1.381 0 2.5-1.119 2.5-2.5 0-1.379-1.119-2.5-2.5-2.5-1.382 0-2.5 1.121-2.5 2.5 0 1.381 1.118 2.5 2.5 2.5zm0-4c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5-1.5-.673-1.5-1.5.673-1.5 1.5-1.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiGroupOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHeadphones")]
        TiIcon::TiHeadphones => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 13c0-4.963-4.037-9-9-9s-9 4.037-9 9v2.6l.023.113c-.013.243-.023.5-.023.787v2c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5v-2c0-1.511-.83-2.79-1.982-3.278.095-2.122 1.837-3.823 3.982-3.823s3.887 1.7 3.982 3.822c-1.152.49-1.982 1.768-1.982 3.279v2c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5v-2c0-.287-.01-.544-.023-.787l.023-.113v-2.6zm-16 0c0-1.586.538-3.046 1.432-4.221l.89.889c-.742.928-1.218 2.075-1.302 3.332-.02 0-.02 1-.02 1h-1v-1zm3 5.5c0 .827-.673 1.5-1.5 1.5s-1.5-.673-1.5-1.5v-2c0-.666.057-1.176.114-1.5h1.886c.473 0 1 .616 1 1.5v2zm7.77-9.338l-.354.354c-.912-.913-2.125-1.416-3.416-1.416s-2.504.503-3.417 1.416l-.354-.354-1.141-1.141-.627-.626c1.479-1.48 3.447-2.295 5.539-2.295 2.093 0 4.06.815 5.539 2.295l-.627.626-1.142 1.141zm3.23 9.338c0 .827-.673 1.5-1.5 1.5s-1.5-.673-1.5-1.5v-2c0-.884.527-1.5 1-1.5h1.886c.057.324.114.834.114 1.5v2zm0-4.5h-1v-1c-.104-1.257-.58-2.404-1.322-3.332l.891-.889c.893 1.175 1.431 2.634 1.431 4.221v1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiHeadphones".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHeart")]
        TiIcon::TiHeart => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 10.375c0-2.416-1.959-4.375-4.375-4.375s-4.375 1.959-4.375 4.375c0 1.127.159 2.784 1.75 4.375l7 5.25s5.409-3.659 7-5.25 1.75-3.248 1.75-4.375c0-2.416-1.959-4.375-4.375-4.375s-4.375 1.959-4.375 4.375\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiHeart".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHeartFullOutline")]
        TiIcon::TiHeartFullOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M2.2 9.4c0 1.3.2 3.3 2 5.1 1.6 1.6 6.9 5.2 7.1 5.4.2.1.4.2.6.2s.4-.1.6-.2c.2-.2 5.5-3.7 7.1-5.4 1.8-1.8 2-3.8 2-5.1 0-3-2.4-5.4-5.4-5.4-1.6 0-3.2.9-4.2 2.3-1-1.4-2.6-2.3-4.4-2.3-2.9 0-5.4 2.4-5.4 5.4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiHeartFullOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHeartHalfOutline")]
        TiIcon::TiHeartHalfOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M2.2 9.4c0 1.3.2 3.3 2 5.1 1.6 1.6 6.9 5.2 7.1 5.4.2.1.4.2.6.2s.4-.1.6-.2c.2-.2 5.5-3.7 7.1-5.4 1.8-1.8 2-3.8 2-5.1 0-3-2.4-5.4-5.4-5.4-1.6 0-3.2.9-4.2 2.3-1-1.4-2.6-2.3-4.4-2.3-2.9 0-5.4 2.4-5.4 5.4zm9.8 1c.6 0 1-.4 1-1 0-1.9 1.5-3.4 3.4-3.4s3.4 1.5 3.4 3.4c0 1.1-.2 2.4-1.5 3.7-1.2 1.2-4.9 3.8-6.3 4.7v-7.4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiHeartHalfOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHeartOutline")]
        TiIcon::TiHeartOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 20c-.195 0-.391-.057-.561-.172-.225-.151-5.508-3.73-7.146-5.371-1.831-1.831-2.043-3.777-2.043-5.082 0-2.964 2.411-5.375 5.375-5.375 1.802 0 3.398.891 4.375 2.256.977-1.365 2.573-2.256 4.375-2.256 2.964 0 5.375 2.411 5.375 5.375 0 1.305-.212 3.251-2.043 5.082-1.641 1.641-6.923 5.22-7.146 5.371-.17.115-.366.172-.561.172zm-4.375-14c-1.861 0-3.375 1.514-3.375 3.375 0 1.093.173 2.384 1.457 3.668 1.212 1.212 4.883 3.775 6.293 4.746 1.41-.971 5.081-3.534 6.293-4.746 1.284-1.284 1.457-2.575 1.457-3.668 0-1.861-1.514-3.375-3.375-3.375s-3.375 1.514-3.375 3.375c0 .552-.447 1-1 1s-1-.448-1-1c0-1.861-1.514-3.375-3.375-3.375z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiHeartOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHome")]
        TiIcon::TiHome => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 3s-6.186 5.34-9.643 8.232c-.203.184-.357.452-.357.768 0 .553.447 1 1 1h2v7c0 .553.447 1 1 1h3c.553 0 1-.448 1-1v-4h4v4c0 .552.447 1 1 1h3c.553 0 1-.447 1-1v-7h2c.553 0 1-.447 1-1 0-.316-.154-.584-.383-.768-3.433-2.892-9.617-8.232-9.617-8.232z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiHome".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHomeOutline")]
        TiIcon::TiHomeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22.262 10.468c-3.39-2.854-9.546-8.171-9.607-8.225l-.655-.563-.652.563c-.062.053-6.221 5.368-9.66 8.248-.438.394-.688.945-.688 1.509 0 1.104.896 2 2 2h1v6c0 1.104.896 2 2 2h12c1.104 0 2-.896 2-2v-6h1c1.104 0 2-.896 2-2 0-.598-.275-1.161-.738-1.532zm-8.262 9.532h-4v-5h4v5zm4-8l.002 8h-3.002v-6h-6v6h-3v-8h-3.001c2.765-2.312 7.315-6.227 9.001-7.68 1.686 1.453 6.234 5.367 9 7.681l-3-.001z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiHomeOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiHtml5")]
        TiIcon::TiHtml5 => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13.1 3.5l.7 1.1.7-1.1v1.5h1v-3h-1l-.7 1.1-.6-1.1h-1.1v3h1zM18.4 5v-1h-1.4v-2h-1v3zM9.8 5h1v-2h.9v-1h-2.8v1h.9zM6.6 4h.9v1h1v-3h-1v1h-.9v-1h-1v3h1zM5 6l1.2 14.4 5.8 1.6 5.8-1.6 1.2-14.4h-14zm11.3 4.6h-6.8l.2 1.8h6.3999999999999995l-.5 5.5-3.6 1-3.6-1-.3-2.9h1.8l.1 1.5 2 .5 2-.5.2-2.3h-6.2l-.5-5.3h9l-.2 1.7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiHtml5".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiImage")]
        TiIcon::TiImage => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<circle cx=\"8.5\" cy=\"8.5\" r=\"2.5\" />\n<path d=\"M16 10c-2 0-3 3-4.5 3s-1.499-1-3.5-1c-2 0-3.001 4-3.001 4h14.001s-1-6-3-6zM20 3h-16c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-12c0-1.103-.897-2-2-2zm0 14h-16v-12h16v12z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiImage".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiImageOutline")]
        TiIcon::TiImageOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8.5 7.999c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5-1.5-.673-1.5-1.5.673-1.5 1.5-1.5m0-1c-1.381 0-2.5 1.119-2.5 2.5s1.119 2.5 2.5 2.5 2.5-1.119 2.5-2.5-1.119-2.5-2.5-2.5zM16 11.999c.45.051 1.27 1.804 1.779 4.001h-11.392c.434-1.034 1.055-2.001 1.612-2.001.806 0 1.125.185 1.53.42.447.258 1.006.58 1.97.58 1.138 0 1.942-.885 2.653-1.666.627-.687 1.218-1.334 1.848-1.334m0-1c-2 0-3 3-4.5 3s-1.499-1-3.5-1c-2 0-3.001 4-3.001 4h14.001s-1-6-3-6zM22 6c0-1.104-.896-2-2-2h-16c-1.104 0-2 .896-2 2v12c0 1.104.896 2 2 2h16c1.104 0 2-.896 2-2v-12zm-2 12h-16v-12h16.003l-.003 12z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiImageOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInfinity")]
        TiIcon::TiInfinity => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.433 8.596c-1.153 0-2.237.449-3.036 1.246l-1.396 1.34-1.375-1.32c-.815-.817-1.901-1.266-3.055-1.266-1.154 0-2.239.451-3.053 1.266-.817.816-1.267 1.9-1.267 3.055 0 1.152.449 2.238 1.266 3.053.814.816 1.899 1.266 3.054 1.266 1.153 0 2.239-.449 3.036-1.248l1.395-1.338 1.376 1.32c.815.816 1.901 1.266 3.055 1.266s2.238-.449 3.053-1.266c.817-.814 1.267-1.9 1.267-3.055s-.449-2.238-1.266-3.055c-.817-.815-1.901-1.264-3.054-1.264zm-7.576 5.605c-.687.688-1.884.688-2.572 0-.344-.344-.533-.801-.533-1.285 0-.486.189-.941.535-1.287.342-.344.799-.533 1.284-.533s.942.189 1.305.551l1.321 1.27-1.34 1.284zm8.861 0c-.687.689-1.866.705-2.59-.018l-1.321-1.27 1.339-1.285c.688-.688 1.886-.688 2.573-.002.344.346.533.801.533 1.287s-.19.944-.534 1.288z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiInfinity".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInfinityOutline")]
        TiIcon::TiInfinityOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.434 8.596c1.152 0 2.237.449 3.053 1.264.815.816 1.266 1.9 1.266 3.056s-.45 2.239-1.268 3.056c-.813.815-1.898 1.266-3.053 1.266s-2.238-.449-3.055-1.266l-1.376-1.32-1.395 1.338c-.797.799-1.882 1.248-3.036 1.248-1.154 0-2.239-.449-3.054-1.267-.815-.813-1.265-1.899-1.265-3.053s.45-2.237 1.267-3.056c.814-.813 1.898-1.266 3.053-1.266 1.154 0 2.239.449 3.055 1.266l1.375 1.32 1.396-1.34c.798-.797 1.882-1.246 3.037-1.246m0-2c-1.679 0-3.25.645-4.433 1.813-1.163-1.159-2.746-1.813-4.43-1.813-1.688 0-3.274.657-4.467 1.853-1.194 1.192-1.852 2.78-1.852 4.469s.658 3.274 1.852 4.468c1.191 1.192 2.779 1.852 4.468 1.852 1.679 0 3.251-.645 4.431-1.814 1.163 1.16 2.746 1.814 4.431 1.814 1.689 0 3.276-.658 4.469-1.854 1.193-1.188 1.852-2.776 1.852-4.467 0-1.688-.658-3.274-1.852-4.47-1.197-1.195-2.783-1.851-4.469-1.851zM7.571 12.096c.225 0 .426.088.612.271l.57.548-.603.579c-.141.142-.352.223-.578.223-.227 0-.438-.08-.58-.223-.155-.155-.24-.36-.24-.578 0-.221.084-.422.243-.581.156-.155.355-.239.576-.239m0-1c-.486 0-.942.189-1.285.533-.345.346-.535.801-.535 1.287 0 .484.189.941.533 1.285.344.344.815.516 1.287.516.471 0 .942-.172 1.285-.516l1.339-1.285-1.321-1.27c-.36-.361-.817-.55-1.303-.55zM16.434 12.113c.228 0 .438.08.576.219.158.159.242.359.242.582s-.083.422-.243.581c-.144.146-.352.228-.571.228-.23 0-.444-.088-.617-.261l-.571-.548.603-.578c.141-.143.353-.223.581-.223m0-1c-.472 0-.943.172-1.287.516l-1.34 1.285 1.322 1.27c.362.361.838.539 1.311.539.472 0 .937-.177 1.279-.521.346-.344.534-.801.534-1.287s-.188-.941-.532-1.287c-.346-.344-.817-.515-1.287-.515z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiInfinityOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInfo")]
        TiIcon::TiInfo => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.17,15.4l-5.91-9.85C14.48,4.25,13.3,3.51,12,3.51S9.52,4.25,8.74,5.54L2.83,15.4c-0.44,0.73-0.66,1.49-0.66,2.21\n\tc0,0.57,0.14,1.13,0.42,1.62C3.23,20.35,4.47,21,6,21h12c1.53,0,2.77-0.65,3.41-1.77c0.28-0.49,0.42-1.02,0.42-1.58\n\tC21.84,16.91,21.62,16.14,21.17,15.4z M12,8.45c0.85,0,1.55,0.7,1.55,1.55c0,0.85-0.69,1.55-1.55,1.55c-0.85,0-1.55-0.7-1.55-1.55\n\tC10.45,9.14,11.14,8.45,12,8.45z M13.69,16.91c-0.03,0.04-0.8,0.92-2.07,0.92l-0.15,0c-0.51-0.03-0.93-0.25-1.18-0.63\n\tc-0.31-0.47-0.36-1.11-0.12-1.82l0.41-1.22c0.23-0.68,0.01-0.79-0.11-0.85l-0.14-0.02c-0.25,0-0.6,0.15-0.71,0.21\n\tc-0.1,0.05-0.23,0.03-0.31-0.07c-0.07-0.1-0.07-0.23,0.01-0.32c0.03-0.04,0.87-0.99,2.22-0.91c0.51,0.03,0.93,0.25,1.18,0.63\n\tc0.32,0.47,0.36,1.11,0.12,1.83l-0.41,1.22c-0.23,0.68-0.01,0.79,0.11,0.85l0.14,0.02c0.25,0,0.6-0.15,0.71-0.2\n\tc0.11-0.06,0.23-0.03,0.31,0.07C13.77,16.69,13.77,16.82,13.69,16.91z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiInfo".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInfoLarge")]
        TiIcon::TiInfoLarge => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13.839 17.525c-.006.002-.559.186-1.039.186-.265 0-.372-.055-.406-.079-.168-.117-.48-.336.054-1.4l1-1.994c.593-1.184.681-2.329.245-3.225-.356-.733-1.039-1.236-1.92-1.416-.317-.065-.639-.097-.958-.097-1.849 0-3.094 1.08-3.146 1.126-.179.158-.221.42-.102.626.12.206.367.3.595.222.005-.002.559-.187 1.039-.187.263 0 .369.055.402.078.169.118.482.34-.051 1.402l-1 1.995c-.594 1.185-.681 2.33-.245 3.225.356.733 1.038 1.236 1.921 1.416.314.063.636.097.954.097 1.85 0 3.096-1.08 3.148-1.126.179-.157.221-.42.102-.626-.12-.205-.369-.297-.593-.223z\" />\n<circle cx=\"13\" cy=\"6.001\" r=\"2.5\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiInfoLarge".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInfoLargeOutline")]
        TiIcon::TiInfoLargeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14.234 16.014l.554-1.104c.808-1.61.897-3.228.253-4.552-.122-.252-.277-.479-.443-.693 1.411-.619 2.402-2.026 2.402-3.664 0-2.206-1.794-4-4-4s-4 1.794-4 4c0 .783.234 1.508.624 2.126-1.696.33-2.806 1.248-2.947 1.375-.716.631-.885 1.68-.405 2.504.324.559.886.909 1.494.98l-.554 1.104c-.808 1.61-.897 3.228-.254 4.552.565 1.164 1.621 1.955 2.972 2.229.413.084.836.127 1.254.127 2.368 0 3.965-1.347 4.14-1.501.716-.63.887-1.678.407-2.503-.325-.556-.887-.909-1.497-.98zm-1.234-12.013c1.104 0 2 .896 2 2s-.896 2-2 2c-1.105 0-2-.896-2-2s.895-2 2-2zm-1.816 14.999c-.271 0-.559-.025-.854-.087-1.642-.334-2.328-1.933-1.328-3.927l1-1.995c.5-.996.47-1.63-.108-2.035-.181-.125-.431-.169-.689-.169-.577 0-1.201.214-1.201.214s1.133-1.001 2.812-1.001c.271 0 .56.025.856.087 1.64.334 2.328 1.933 1.328 3.927l-1 1.993c-.5.998-.472 1.632.106 2.035.181.126.433.169.692.169.577 0 1.2-.212 1.2-.212s-1.133 1.001-2.814 1.001z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiInfoLargeOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInfoOutline")]
        TiIcon::TiInfoOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12,5.51c0.56,0,1.12,0.35,1.54,1.06l5.91,9.85C20.31,17.84,19.65,19,18,19H6c-1.65,0-2.31-1.16-1.46-2.57l5.91-9.85\n\tC10.88,5.86,11.44,5.51,12,5.51 M12,3.51c-1.3,0-2.48,0.74-3.26,2.03L2.83,15.4c-0.44,0.74-0.66,1.5-0.66,2.23\n\tc0,0.56,0.14,1.11,0.42,1.6C3.23,20.35,4.47,21,6,21h12c1.53,0,2.77-0.65,3.41-1.77c0.29-0.51,0.43-1.07,0.42-1.65\n\tc-0.01-0.71-0.23-1.46-0.66-2.18l-5.91-9.85C14.48,4.25,13.3,3.51,12,3.51z M13.5,16.75c0,0-0.71,0.36-1.07,0.18\n\tc-0.36-0.18-0.43-0.54-0.23-1.15l0.41-1.22c0.4-1.22-0.12-2.08-1.08-2.13c-1.26-0.07-2.02,0.83-2.02,0.83s0.71-0.36,1.07-0.18\n\tc0.36,0.18,0.43,0.54,0.23,1.15l-0.41,1.22c-0.4,1.22,0.12,2.07,1.08,2.13C12.74,17.64,13.5,16.75,13.5,16.75z\" />\n<circle cx=\"12\" cy=\"10\" r=\"1.3\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiInfoOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInputChecked")]
        TiIcon::TiInputChecked => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16 19h-8c-1.654 0-3-1.346-3-3v-8c0-1.654 1.346-3 3-3h5c.553 0 1 .448 1 1s-.447 1-1 1h-5c-.552 0-1 .449-1 1v8c0 .551.448 1 1 1h8c.552 0 1-.449 1-1v-3c0-.552.447-1 1-1s1 .448 1 1v3c0 1.654-1.346 3-3 3zM13.166 14.833c-.35 0-.689-.139-.941-.391l-2.668-2.667c-.52-.521-.52-1.365 0-1.885.521-.521 1.365-.521 1.887 0l1.416 1.417 3.475-5.455c.357-.644 1.17-.877 1.814-.519.643.358.875 1.17.518 1.813l-4.334 7c-.203.366-.566.615-.98.673l-.187.014z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiInputChecked".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiInputCheckedOutline")]
        TiIcon::TiInputCheckedOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.885 6.177c-.221-.771-.728-1.409-1.427-1.798-.445-.248-.949-.379-1.457-.379-.862 0-1.661.381-2.219 1h-6.782c-1.654 0-3 1.346-3 3v8c0 1.654 1.346 3 3 3h8c1.654 0 3-1.346 3-3v-6.454l.622-1.088c.39-.7.482-1.51.263-2.281zm-3.758.338c.104-.189.27-.328.459-.416.301-.113.623-.127.9.027.232.13.402.343.476.6.033.117.039.236.03.353-.012.115-.043.227-.092.332l-.021.065-4.006 7.011c-.151.273-.427.461-.742.506l-.132.009c-.267 0-.518-.104-.707-.293l-3-3c-.39-.39-.39-1.024 0-1.414.189-.189.44-.293.707-.293s.518.104.707.293l1.125 1.125.92.92.652-1.125 2.724-4.7zm-.127 10.485h-8c-.552 0-1-.449-1-1v-8c0-.551.448-1 1-1h6.689l-2.15 3.712-1.125-1.125c-.391-.391-.902-.586-1.414-.586s-1.023.195-1.414.586c-.781.781-.781 2.047 0 2.828l3 3c.378.378.888.586 1.414.586l.277-.02c.621-.087 1.166-.461 1.471-1.01l2.252-3.94v4.969c0 .551-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiInputCheckedOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiKey")]
        TiIcon::TiKey => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8.5 11c0 .732.166 1.424.449 2.051l-3.949 3.949v1.5s.896 1.5 2 1.5h2v-2h2v-2h2.5c2.762 0 5-2.238 5-5s-2.238-5-5-5-5 2.238-5 5zm5 2c-1.104 0-2-.896-2-2 0-1.105.896-2.002 2-2.002 1.105 0 2 .896 2 2.002 0 1.104-.895 2-2 2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiKey".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiKeyOutline")]
        TiIcon::TiKeyOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10 21h-6v-4.414l3.783-3.783c-.187-.587-.283-1.192-.283-1.803 0-3.309 2.691-6 6-6s6 2.691 6 6-2.691 6-6 6h-1.5v2h-2v2zm-4-2h2v-2h2v-2h3.5c2.206 0 4-1.794 4-4s-1.794-4-4-4-4 1.794-4 4c0 .559.121 1.109.359 1.639l.285.631-4.144 4.144v1.586zM13.5 9.998c.551 0 1 .449 1 1.002 0 .552-.449 1-1 1s-1-.448-1-1c0-.553.449-1.002 1-1.002m0-1c-1.104 0-2 .896-2 2.002 0 1.104.896 2 2 2 1.105 0 2-.896 2-2 0-1.105-.895-2.002-2-2.002z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiKeyOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiKeyboard")]
        TiIcon::TiKeyboard => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 13h7v2h-7zM5 13h2v2h-2zM5 9h2v1h-2zM8 12v-1h-3v1h2zM8 9h1v1h-1zM9 11h1v1h-1zM10 9h1v1h-1zM11 11h1v1h-1zM12 9h1v1h-1zM13 11h1v1h-1zM14 9h1v1h-1zM15 11h1v1h-1zM16 9h1v1h-1zM17 12h2v-3h-1v2h-1zM18 13h-1v1h-1v1h3v-1h-1zM20 6h-16c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2zm0 10h-16v-8h16v8z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiKeyboard".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLeaf")]
        TiIcon::TiLeaf => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20 11c0-4.9-3.499-9.1-8.32-9.983l-.18-.034-.18.033c-4.821.884-8.32 5.084-8.32 9.984 0 4.617 3.108 8.61 7.5 9.795v1.205c0 .553.448 1 1 1s1-.447 1-1v-1.205c4.392-1.185 7.5-5.178 7.5-9.795zm-7.5 7.7v-2.993l4.354-4.354c.195-.195.195-.512 0-.707s-.512-.195-.707 0l-3.647 3.647v-3.586l2.354-2.354c.195-.195.195-.512 0-.707s-.512-.195-.707 0l-1.647 1.647v-3.293c0-.553-.448-1-1-1s-1 .447-1 1v3.293l-1.646-1.647c-.195-.195-.512-.195-.707 0s-.195.512 0 .707l2.354 2.354v3.586l-3.646-3.646c-.195-.195-.512-.195-.707 0s-.195.512 0 .707l4.354 4.354v2.992c-3.249-1.116-5.502-4.179-5.502-7.7 0-3.874 2.723-7.201 6.5-7.981 3.777.78 6.5 4.107 6.5 7.981 0 3.521-2.253 6.584-5.5 7.7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLeaf".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLightbulb")]
        TiIcon::TiLightbulb => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12.5 5.5c-.276 0-.5.224-.5.5s.224.5.5.5c1.083 0 1.964.881 1.964 1.964 0 .276.224.5.5.5s.5-.224.5-.5c0-1.634-1.33-2.964-2.964-2.964zM12.5 1c-4.136 0-7.5 3.364-7.5 7.5 0 1.486.44 2.922 1.274 4.165l.08.135c1.825 2.606 2.146 3.43 2.146 4.2v3c0 .552.448 1 1 1h2c0 .26.11.52.29.71.19.18.45.29.71.29.26 0 .52-.11.71-.29.18-.19.29-.45.29-.71h2c.552 0 1-.448 1-1v-3c0-.782.319-1.61 2.132-4.199.895-1.275 1.368-2.762 1.368-4.301 0-4.136-3.364-7.5-7.5-7.5zm2 18h-4v-1h4v1zm2.495-7.347c-1.466 2.093-2.143 3.289-2.385 4.347h-1.11v-2c0-.552-.448-1-1-1s-1 .448-1 1v2h-1.113c-.24-1.03-.898-2.2-2.306-4.22l-.077-.129c-.657-.934-1.004-2.024-1.004-3.151 0-3.033 2.467-5.5 5.5-5.5s5.5 2.467 5.5 5.5c0 1.126-.347 2.216-1.005 3.153z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLightbulb".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLink")]
        TiIcon::TiLink => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.277 6.321c-.43-.43-1.126-.43-1.556 0l-1.721 1.722-.308-.308c-1.168-1.168-3.216-1.168-4.384 0l-4.172 4.172c-.584.584-.906 1.363-.906 2.192s.322 1.608.906 2.192l.308.308-1.722 1.722c-.43.43-.43 1.126 0 1.556.215.215.496.322.778.322s.563-.107.778-.322l1.722-1.722.308.308c.584.584 1.362.906 2.192.906s1.608-.322 2.192-.906l4.172-4.172c.584-.584.906-1.362.906-2.192s-.322-1.608-.906-2.192l-.308-.308 1.722-1.722c.429-.43.429-1.126-.001-1.556zm-2.969 6.414l-4.172 4.172c-.168.168-.402.253-.636.253s-.468-.084-.636-.253l-.308-.308.722-.722c.43-.43.43-1.126 0-1.556-.215-.215-.496-.322-.778-.322s-.563.107-.778.322l-.722.722-.308-.308c-.168-.168-.261-.395-.261-.636s.093-.468.261-.636l4.172-4.172c.168-.168.394-.261.636-.261s.468.093.636.261l.308.308-.722.722c-.43.43-.43 1.126 0 1.556.215.215.496.322.778.322s.563-.107.778-.322l.722-.722.308.308c.168.168.261.395.261.636s-.093.468-.261.636z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLink".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLinkOutline")]
        TiIcon::TiLinkOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.5 5.999c.282 0 .562.106.777.321.431.431.431 1.127 0 1.557l-1.72 1.723.307.308c.585.584.906 1.362.906 2.192s-.321 1.607-.906 2.191l-4.172 4.172c-.584.584-1.361.906-2.191.906s-1.607-.322-2.191-.906l-.31-.309-1.723 1.723c-.215.215-.495.322-.777.322s-.562-.107-.777-.322c-.431-.43-.431-1.126 0-1.557l1.72-1.72-.308-.309c-.583-.584-.905-1.361-.905-2.191s.321-1.608.905-2.192l4.173-4.173c.584-.584 1.387-.875 2.191-.875s1.607.291 2.191.875l.31.308 1.723-1.723c.215-.215.495-.321.777-.321m0-2c-.828 0-1.605.321-2.191.908l-.492.491c-.707-.351-1.504-.539-2.316-.539-1.363 0-2.677.533-3.605 1.461l-4.172 4.173c-.964.962-1.494 2.241-1.494 3.607 0 .822.192 1.616.558 2.327l-.479.48c-.586.585-.909 1.364-.909 2.193 0 .827.322 1.605.908 2.191.584.586 1.363.908 2.191.908s1.605-.322 2.191-.908l.48-.48c.711.365 1.504.559 2.328.559 1.363 0 2.645-.53 3.605-1.492l4.172-4.172c.963-.962 1.492-2.242 1.492-3.605 0-.824-.192-1.617-.558-2.328l.479-.48c.589-.587.912-1.366.912-2.193 0-.828-.322-1.606-.908-2.192-.587-.588-1.364-.909-2.192-.909zM11.4 11.168c.017.535.233 1.036.613 1.416.381.38.881.598 1.416.614l-1.832 1.831c-.017-.534-.234-1.035-.613-1.415-.381-.38-.881-.597-1.416-.614l1.832-1.832m1.1-2.139c-.242 0-.468.094-.637.262l-4.172 4.172c-.168.168-.26.395-.26.637 0 .24.092.467.26.635l.309.308.723-.723c.215-.215.495-.321.777-.321s.562.106.777.321c.431.431.431 1.127 0 1.557l-.72.723.308.308c.168.168.401.253.636.253s.468-.084.637-.253l4.172-4.173c.168-.168.26-.395.26-.635 0-.242-.092-.469-.26-.637l-.31-.309-.723.723c-.215.215-.495.322-.777.322s-.562-.107-.777-.322c-.431-.43-.431-1.126 0-1.557l.72-.72-.307-.309c-.169-.168-.395-.262-.636-.262z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLinkOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLocation")]
        TiIcon::TiLocation => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.657 5.304c-3.124-3.073-8.189-3.073-11.313 0-3.124 3.074-3.124 8.057 0 11.13l5.656 5.565 5.657-5.565c3.124-3.073 3.124-8.056 0-11.13zm-5.657 8.195c-.668 0-1.295-.26-1.768-.732-.975-.975-.975-2.561 0-3.536.472-.472 1.1-.732 1.768-.732s1.296.26 1.768.732c.975.975.975 2.562 0 3.536-.472.472-1.1.732-1.768.732z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLocation".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLocationArrow")]
        TiIcon::TiLocationArrow => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.368 19.102c.349 1.049 1.011 1.086 1.478.086l5.309-11.375c.467-1.002.034-1.434-.967-.967l-11.376 5.308c-1.001.467-.963 1.129.085 1.479l4.103 1.367 1.368 4.102z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLocationArrow".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLocationArrowOutline")]
        TiIcon::TiLocationArrowOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.087 20.914c-.353 0-1.219-.146-1.668-1.496l-1.209-3.627-3.628-1.209c-1.244-.415-1.469-1.172-1.493-1.587s.114-1.193 1.302-1.747l11.375-5.309c1.031-.479 1.922-.309 2.348.362.224.351.396.97-.053 1.933l-5.309 11.375c-.529 1.135-1.272 1.305-1.665 1.305zm-5.39-8.068l4.094 1.363 1.365 4.093 4.775-10.233-10.234 4.777z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiLocationArrowOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLocationOutline")]
        TiIcon::TiLocationOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 5c1.609 0 3.12.614 4.254 1.73 1.126 1.107 1.746 2.579 1.746 4.14s-.62 3.03-1.745 4.139l-4.255 4.184-4.254-4.186c-1.125-1.107-1.745-2.576-1.745-4.139s.62-3.032 1.745-4.141c1.135-1.113 2.647-1.727 4.254-1.727m0-2c-2.047 0-4.096.768-5.657 2.305-3.124 3.074-3.124 8.057 0 11.131l5.657 5.563 5.657-5.565c3.124-3.072 3.124-8.056 0-11.129-1.561-1.537-3.609-2.305-5.657-2.305zM12 8.499c.668 0 1.296.26 1.768.731.976.976.976 2.562 0 3.537-.473.472-1.1.731-1.768.731s-1.295-.26-1.768-.731c-.976-.976-.976-2.562 0-3.537.473-.471 1.101-.731 1.768-.731m0-1c-.896 0-1.792.342-2.475 1.024-1.367 1.367-1.367 3.584 0 4.951.684.684 1.578 1.024 2.475 1.024s1.792-.342 2.475-1.024c1.366-1.367 1.366-3.584 0-4.951-.683-.683-1.579-1.024-2.475-1.024z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiLocationOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLockClosed")]
        TiIcon::TiLockClosed => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 10h-1v-2c0-2.205-1.794-4-4-4s-4 1.795-4 4v2h-1c-1.103 0-2 .896-2 2v7c0 1.104.897 2 2 2h10c1.103 0 2-.896 2-2v-7c0-1.104-.897-2-2-2zm-5 8.299c-.719 0-1.3-.58-1.3-1.299s.581-1.301 1.3-1.301 1.3.582 1.3 1.301-.581 1.299-1.3 1.299zm2-7.299h-4v-3c0-1.104.897-2 2-2s2 .896 2 2v3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLockClosed".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLockClosedOutline")]
        TiIcon::TiLockClosedOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<circle cx=\"12\" cy=\"17\" r=\"1.3\" />\n<path d=\"M17 10h-1v-2c0-2.206-1.794-4-4-4s-4 1.794-4 4v2h-1c-1.104 0-2 .896-2 2v7c0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2v-7c0-1.104-.896-2-2-2zm-7-2c0-1.104.896-2 2-2s2 .896 2 2v3h-4v-3zm7 11h-10v-7h10.003l-.003 7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiLockClosedOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLockOpen")]
        TiIcon::TiLockOpen => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 4c-2.206 0-4 1.795-4 4v3h-4v-1h-3c-1.103 0-2 .896-2 2v7c0 1.104.897 2 2 2h10c1.103 0 2-.896 2-2v-7c0-1.104-.897-2-2-2h-1v-2c0-1.104.897-2 2-2s2 .896 2 2v3c0 .553.448 1 1 1s1-.447 1-1v-3c0-2.205-1.794-4-4-4zm-6 14.299c-.719 0-1.3-.58-1.3-1.299s.581-1.301 1.3-1.301 1.3.582 1.3 1.301-.581 1.299-1.3 1.299z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiLockOpen".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiLockOpenOutline")]
        TiIcon::TiLockOpenOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<circle cx=\"12\" cy=\"17\" r=\"1.3\" />\n<path d=\"M18 4c-2.206 0-4 1.794-4 4v3h-4v-1h-3c-1.104 0-2 .896-2 2v7c0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2v-7c0-1.104-.896-2-2-2h-1v-2c0-1.104.896-2 2-2s2 .896 2 2v3c0 .552.448 1 1 1s1-.448 1-1v-3c0-2.206-1.794-4-4-4zm-1 15h-10v-7h10.003l-.003 7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiLockOpenOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMail")]
        TiIcon::TiMail => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 7h-14c-1.104 0-2 .896-2 2v9c0 1.104.896 2 2 2h14c1.104 0 2-.896 2-2v-9c0-1.104-.896-2-2-2zm-9.684 7.316l1.602 1.4c.305.266.691.398 1.082.398s.777-.133 1.082-.398l1.602-1.4-.037.037 3.646 3.646h-12.586l3.646-3.646-.037-.037zm-4.316 2.977v-6.753l3.602 3.151-3.602 3.602zm10.398-3.602l3.602-3.151v6.75l-3.602-3.599zm3.602-4.691v.21l-6.576 5.754c-.227.198-.621.198-.848 0l-6.576-5.754v-.21h14z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMail".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMap")]
        TiIcon::TiMap => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.383 3.076c-.373-.155-.804-.069-1.09.217l-3.867 3.867-4.301-3.441c-.396-.316-.973-.287-1.332.074l-4.5 4.5c-.188.187-.293.441-.293.707v10c0 .404.243.77.617.924.124.053.254.076.383.076.26 0 .516-.102.707-.293l3.867-3.867 4.301 3.441c.396.316.971.285 1.332-.074l4.5-4.5c.188-.187.293-.441.293-.707v-10c0-.404-.243-.77-.617-.924zm-13.383 13.51v-7.172l3-3v7.24c-.07.043-3 2.932-3 2.932zm4.125-2.867l-.125-.068v-7.469s3.959 3.143 4 3.166v7.473l-3.875-3.102zm7.875-.133l-3 3v-7.236c.07-.043 3-2.936 3-2.936v7.172z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMap".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaEject")]
        TiIcon::TiMediaEject => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 16h-10c-1.104 0-2 .895-2 2 0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2 0-1.105-.896-2-2-2zM18.433 10.604c-2.574-2.641-6.433-6.604-6.433-6.604s-3.859 3.963-6.433 6.604c-.349.363-.567.853-.567 1.396 0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2 0-.543-.218-1.033-.567-1.396z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMediaEject".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaEjectOutline")]
        TiIcon::TiMediaEjectOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M16 21h-8c-1.654 0-3-1.346-3-3s1.346-3 3-3h8c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-8-4c-.551 0-1 .448-1 1s.449 1 1 1h8c.551 0 1-.448 1-1s-.449-1-1-1h-8zM12 6.866l4.964 5.096.036.038-10 .004.08-.087 4.92-5.051m0-2.866s-3.859 3.963-6.433 6.604c-.349.363-.567.853-.567 1.396 0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2 0-.543-.218-1.033-.568-1.393-2.573-2.644-6.432-6.607-6.432-6.607z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaEjectOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaFastForward")]
        TiIcon::TiMediaFastForward => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15.053 6.912c-.324-.314-.765-.512-1.253-.512-.994 0-1.8.807-1.8 1.801v9c0 .994.806 1.799 1.8 1.799.488 0 .93-.195 1.253-.512 2.381-2.314 5.947-5.787 5.947-5.787s-3.566-3.474-5.947-5.789zM6.053 6.912c-.324-.314-.765-.512-1.253-.512-.994 0-1.8.807-1.8 1.801v9c0 .994.806 1.799 1.8 1.799.488 0 .93-.195 1.253-.512 2.381-2.314 5.947-5.787 5.947-5.787s-3.566-3.474-5.947-5.789z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaFastForward".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaFastForwardOutline")]
        TiIcon::TiMediaFastForwardOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M14 8.676l4.133 4.025-4.133 4.026v-8.051m-.2-2.276c-.994 0-1.8.807-1.8 1.801v9c0 .994.806 1.799 1.8 1.799.488 0 .93-.195 1.253-.512 2.381-2.314 5.947-5.787 5.947-5.787s-3.566-3.475-5.944-5.789c-.327-.314-.768-.512-1.256-.512zM5 8.676l4.133 4.025-4.133 4.026v-8.051m-.2-2.276c-.994 0-1.8.807-1.8 1.801v9c0 .994.806 1.799 1.8 1.799.488 0 .93-.195 1.253-.512 2.381-2.314 5.947-5.787 5.947-5.787s-3.566-3.474-5.944-5.789c-.327-.314-.768-.512-1.256-.512z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiMediaFastForwardOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaPause")]
        TiIcon::TiMediaPause => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 6c-1.104 0-2 .896-2 2v8c0 1.104.896 2 2 2s2-.896 2-2v-8c0-1.104-.896-2-2-2zM15 6c-1.104 0-2 .896-2 2v8c0 1.104.896 2 2 2s2-.896 2-2v-8c0-1.104-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMediaPause".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaPauseOutline")]
        TiIcon::TiMediaPauseOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 20c-1.654 0-3-1.346-3-3v-9c0-1.654 1.346-3 3-3s3 1.346 3 3v9c0 1.654-1.346 3-3 3zm0-13c-.552 0-1 .449-1 1v9c0 .551.448 1 1 1s1-.449 1-1v-9c0-.551-.448-1-1-1zM15 20c-1.654 0-3-1.346-3-3v-9c0-1.654 1.346-3 3-3s3 1.346 3 3v9c0 1.654-1.346 3-3 3zm0-13c-.552 0-1 .449-1 1v9c0 .551.448 1 1 1s1-.449 1-1v-9c0-.551-.448-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaPauseOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaPlay")]
        TiIcon::TiMediaPlay => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.396 18.433c2.641-2.574 6.604-6.433 6.604-6.433s-3.963-3.859-6.604-6.433c-.363-.349-.853-.567-1.396-.567-1.104 0-2 .896-2 2v10c0 1.104.896 2 2 2 .543 0 1.033-.218 1.396-.567z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMediaPlay".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaPlayOutline")]
        TiIcon::TiMediaPlayOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8.998 7.002l.085.078 5.051 4.92-5.096 4.964-.038.036-.002-9.998m.002-2.002c-1.104 0-2 .896-2 2v10c0 1.104.896 2 2 2 .543 0 1.033-.218 1.393-.568 2.644-2.573 6.607-6.432 6.607-6.432s-3.963-3.859-6.604-6.433c-.363-.349-.853-.567-1.396-.567z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaPlayOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaPlayReverse")]
        TiIcon::TiMediaPlayReverse => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14 19c1.1 0 2-.9 2-2v-10c0-1.1-.9-2-2-2-.5 0-1 .2-1.4.6-2.6 2.5-6.6 6.4-6.6 6.4s4 3.9 6.6 6.4c.4.4.9.6 1.4.6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaPlayReverse".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaPlayReverseOutline")]
        TiIcon::TiMediaPlayReverseOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14 7v10l-5.1-5 5.1-5m-1.4-1.4c-2.6 2.5-6.6 6.4-6.6 6.4s4 3.9 6.6 6.4c.4.4.9.6 1.4.6 1.1 0 2-.9 2-2v-10c0-1.1-.9-2-2-2-.5 0-1 .2-1.4.6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiMediaPlayReverseOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaRecord")]
        TiIcon::TiMediaRecord => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 12c0-1.657-.672-3.157-1.757-4.243-1.086-1.085-2.586-1.757-4.243-1.757-1.656 0-3.156.672-4.242 1.757-1.086 1.086-1.758 2.586-1.758 4.243 0 1.656.672 3.156 1.758 4.242s2.586 1.758 4.242 1.758c1.657 0 3.157-.672 4.243-1.758 1.085-1.086 1.757-2.586 1.757-4.242z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMediaRecord".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaRecordOutline")]
        TiIcon::TiMediaRecordOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 8c2.205 0 4 1.794 4 4s-1.795 4-4 4-4-1.794-4-4 1.795-4 4-4m0-2c-3.314 0-6 2.686-6 6 0 3.312 2.686 6 6 6 3.312 0 6-2.688 6-6 0-3.314-2.688-6-6-6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaRecordOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaRewind")]
        TiIcon::TiMediaRewind => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.2 6.4c-.488 0-.931.197-1.253.512-2.381 2.315-5.947 5.789-5.947 5.789l5.944 5.789c.326.315.768.51 1.256.51.994 0 1.8-.805 1.8-1.799v-9c0-.994-.806-1.801-1.8-1.801zM19.2 6.4c-.488 0-.931.197-1.253.512-2.381 2.315-5.947 5.789-5.947 5.789l5.944 5.789c.326.315.768.51 1.256.51.994 0 1.8-.805 1.8-1.799v-9c0-.994-.806-1.801-1.8-1.801z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMediaRewind".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaRewindOutline")]
        TiIcon::TiMediaRewindOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M10 8.676v8.05l-4.133-4.025 4.133-4.025m.2-2.276c-.488 0-.931.197-1.253.512-2.381 2.315-5.947 5.789-5.947 5.789l5.944 5.789c.326.315.768.51 1.256.51.994 0 1.8-.805 1.8-1.799v-9c0-.994-.806-1.801-1.8-1.801zM19 8.676v8.051l-4.133-4.025 4.133-4.026m.2-2.276c-.488 0-.931.197-1.253.512-2.381 2.315-5.947 5.789-5.947 5.789l5.944 5.789c.326.315.768.51 1.256.51.994 0 1.8-.805 1.8-1.799v-9c0-.994-.806-1.801-1.8-1.801z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaRewindOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaStop")]
        TiIcon::TiMediaStop => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16 6h-8c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMediaStop".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMediaStopOutline")]
        TiIcon::TiMediaStopOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16 8v8h-8v-8h8m0-2h-8c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMediaStopOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMessage")]
        TiIcon::TiMessage => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 7c.542 0 1 .458 1 1v7c0 .542-.458 1-1 1h-8.829l-.171.171v-.171h-3c-.542 0-1-.458-1-1v-7c0-.542.458-1 1-1h12m0-2h-12c-1.65 0-3 1.35-3 3v7c0 1.65 1.35 3 3 3h1v3l3-3h8c1.65 0 3-1.35 3-3v-7c0-1.65-1.35-3-3-3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMessage".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMessageTyping")]
        TiIcon::TiMessageTyping => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 6h-13c-1.65 0-3 1.35-3 3v7c0 1.65 1.35 3 3 3h1v3l3-3h9c1.65 0 3-1.35 3-3v-7c0-1.65-1.35-3-3-3zm1 10c0 .542-.458 1-1 1h-13c-.542 0-1-.458-1-1v-7c0-.542.458-1 1-1h13c.542 0 1 .458 1 1v7zM7 14.5c-1.104 0-2-.896-2-2s.896-2 2-2 2 .896 2 2-.896 2-2 2zm0-3c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zM11.5 14.5c-1.104 0-2-.896-2-2s.896-2 2-2 2 .896 2 2-.896 2-2 2zm0-3c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zM16 14.5c-1.104 0-2-.896-2-2s.896-2 2-2 2 .896 2 2-.896 2-2 2zm0-3c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMessageTyping".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMessages")]
        TiIcon::TiMessages => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 7h-3c0-1.65-1.35-3-3-3h-12c-1.65 0-3 1.35-3 3v7c0 1.65 1.35 3 3 3v3l3-3c0 1.65 1.35 3 3 3h8l3 3v-3h1c1.65 0 3-1.35 3-3v-7c0-1.65-1.35-3-3-3zm-18 8c-.542 0-1-.458-1-1v-7c0-.542.458-1 1-1h12c.542 0 1 .458 1 1v1h-6.5c-1.379 0-2.5 1.121-2.5 2.5v4.5h-4zm19 2c0 .542-.458 1-1 1h-12c-.542 0-1-.458-1-1v-6.5c0-.827.673-1.5 1.5-1.5h11.5c.542 0 1 .458 1 1v7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMessages".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMicrophone")]
        TiIcon::TiMicrophone => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 16c2.206 0 4-1.795 4-4v-6c0-2.206-1.794-4-4-4s-4 1.794-4 4v6c0 2.205 1.794 4 4 4zM19 12v-2c0-.552-.447-1-1-1s-1 .448-1 1v2c0 2.757-2.243 5-5 5s-5-2.243-5-5v-2c0-.552-.447-1-1-1s-1 .448-1 1v2c0 3.52 2.613 6.432 6 6.92v1.08h-3c-.553 0-1 .447-1 1s.447 1 1 1h8c.553 0 1-.447 1-1s-.447-1-1-1h-3v-1.08c3.387-.488 6-3.4 6-6.92z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMicrophone".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMicrophoneOutline")]
        TiIcon::TiMicrophoneOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 16c-2.206 0-4-1.795-4-4v-6c0-2.205 1.794-4 4-4s4 1.795 4 4v6c0 2.205-1.794 4-4 4zm0-12c-1.103 0-2 .896-2 2v6c0 1.104.897 2 2 2s2-.896 2-2v-6c0-1.104-.897-2-2-2zM19 12v-2c0-.553-.447-1-1-1s-1 .447-1 1v2c0 2.757-2.243 5-5 5s-5-2.243-5-5v-2c0-.553-.447-1-1-1s-1 .447-1 1v2c0 3.52 2.613 6.432 6 6.92v1.08h-3c-.553 0-1 .447-1 1s.447 1 1 1h8c.553 0 1-.447 1-1s-.447-1-1-1h-3v-1.08c3.387-.488 6-3.4 6-6.92z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiMicrophoneOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMinus")]
        TiIcon::TiMinus => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 11h-12c-1.104 0-2 .896-2 2s.896 2 2 2h12c1.104 0 2-.896 2-2s-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMinus".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMinusOutline")]
        TiIcon::TiMinusOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 16h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.551 0-1 .449-1 1s.449 1 1 1h12c.551 0 1-.449 1-1s-.449-1-1-1h-12z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMinusOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiMortarBoard")]
        TiIcon::TiMortarBoard => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.5 7.9s-5.8-2.9-6.9-3.5-1.7-.7-3 0c-1.3.6-6.7 3.3-6.7 3.3-.8.4-1.5 1.2-1.5 2s.2 1.2.2 1.2-.1.3-.3 1.5c-.3 1.2-.3 2.7-.3 3.3 0 1.5 1.3 2.6 2.2 2.7.9.1 1.6-.1 1.6-.1 1.4 1.3 3.7 2.1 6.4 2.1 4.4 0 7.8-2.2 7.8-5 0-1.1-.4-2.7-.4-2.7l1.1-.6c.9-.5 1.3-1.4 1.3-2.3-.1-.8-.6-1.5-1.5-1.9zm-8.2 10.4c-3.2 0-5.8-1.3-5.8-3l.5-2.8 4.2 2.1c.6.3 1.5.3 2.2 0l4.3-2.1.4 2.8c0 1.6-2.5 3-5.8 3zm7.3-8.1l-6.6 3.4c-.4.2-1 .2-1.4 0l-5.7-2.9c-.2.5-.3 1.2-.3 2 0 1.4.2 2.4.2 2.9s-.3.8-.7.8h-.1c-.4 0-.8-.3-.8-.8s0-1.6.3-3.1c.2-.9.4-1.7.6-2.2l-.2-.1c-.4-.2-.4-.5 0-.7l6.7-3.4c.4-.2.9-.2 1.3 0s6.7 3.4 6.7 3.4c.4.2.4.5 0 .7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiMortarBoard".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiNews")]
        TiIcon::TiNews => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 4h-18c-1.104 0-2 .896-2 2v12c0 1.104.896 2 2 2h18c1.104 0 2-.896 2-2v-12c0-1.104-.896-2-2-2zm-18 2h8v12h-8v-12zm18 12h-9v-12h9.003l-.003 12zM20 13.5c0-.275-.225-.5-.5-.5h-1c-.275 0-.5.225-.5.5v3c0 .275.225.5.5.5h1c.275 0 .5-.225.5-.5v-3zM17 7.5c0-.275-.225-.5-.5-.5h-3c-.275 0-.5.225-.5.5v5c0 .275.225.5.5.5h3c.275 0 .5-.225.5-.5v-5zM18.5 10h1c.275 0 .5-.225.5-.5s-.225-.5-.5-.5h-1c-.275 0-.5.225-.5.5s.225.5.5.5zM18.5 12h1c.275 0 .5-.225.5-.5s-.225-.5-.5-.5h-1c-.275 0-.5.225-.5.5s.225.5.5.5zM13.5 15h3c.275 0 .5-.225.5-.5s-.225-.5-.5-.5h-3c-.275 0-.5.225-.5.5s.225.5.5.5zM16.5 16h-3c-.275 0-.5.225-.5.5s.225.5.5.5h3c.275 0 .5-.225.5-.5s-.225-.5-.5-.5zM18.5 8h1c.275 0 .5-.225.5-.5s-.225-.5-.5-.5h-1c-.275 0-.5.225-.5.5s.225.5.5.5zM10 7.5c0-.275-.225-.5-.5-.5h-5c-.275 0-.5.225-.5.5v3c0 .275.225.5.5.5h5c.275 0 .5-.225.5-.5v-3zM9.501 14h-5c-.274 0-.5.225-.5.5s.226.5.5.5h5c.274 0 .499-.225.499-.5s-.225-.5-.499-.5zM9.501 12h-5c-.274 0-.5.225-.5.5s.226.5.5.5h5c.274 0 .499-.225.499-.5s-.225-.5-.499-.5zM9.501 16h-5c-.274 0-.5.225-.5.5s.226.5.5.5h5c.274 0 .499-.225.499-.5s-.225-.5-.499-.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiNews".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiNotes")]
        TiIcon::TiNotes => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.831 4.059c-.107-.095-.243-.139-.394-.121l-11 1.25c-.249.031-.437.244-.437.496v10.316c-1.654 0-3 1.122-3 2.5s1.346 2.5 3 2.5 3-1.122 3-2.5v-7.609l6-.625v3.734c-1.654 0-3 1.122-3 2.5s1.346 2.5 3 2.5 3-1.122 3-2.5v-12.066c0-.144-.062-.28-.169-.375z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiNotes".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiNotesOutline")]
        TiIcon::TiNotesOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.324 4.367c-.368-.324-.84-.5-1.324-.5l-.248.016-9 1.25c-1.001.125-1.752.975-1.752 1.984v6.111c-1.746.551-3 2.034-3 3.772 0 2.205 2.019 4 4.5 4 1.695 0 3.169-.842 3.937-2.078.803.667 1.879 1.078 3.063 1.078 2.481 0 4.5-1.795 4.5-4v-10.133c0-.574-.246-1.119-.676-1.5zm-7.324 11.633v-4.256l3-.45v1.737c-1.693.208-3 1.46-3 2.969zm6 0c0 1.104-1.119 2-2.5 2s-2.5-.896-2.5-2 1.119-2 2.5-2c.172 0 .338.014.5.041v-3.908l-5 .75v6.117c0 1.104-1.119 2-2.5 2s-2.5-.896-2.5-2 1.119-2 2.5-2c.172 0 .338.014.5.041v-7.924l9-1.25v10.133z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiNotesOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPen")]
        TiIcon::TiPen => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.329 7.207c0-1.212-.472-2.352-1.329-3.207s-1.996-1.329-3.207-1.329c-1.199 0-2.327.463-3.18 1.304-.027.025-7.967 7.964-7.967 7.964-.373.373-.717.91-.967 1.514-.195.473-1.979 5.863-2.336 6.939-.119.358-.025.754.242 1.021.189.189.445.293.707.293.105 0 .211-.018.314-.051 1.076-.355 6.465-2.141 6.938-2.336.603-.248 1.14-.592 1.515-.967l2.157-2.156.076.01c.64 0 1.28-.244 1.769-.732l4.5-4.5c.696-.695.887-1.699.588-2.572.107-.386.18-.783.18-1.195zm-11.864 10.379c-.406.143-1.145.393-2.038.691l-1.704-1.704c.301-.894.551-1.634.691-2.038l3.051 3.051zm-4.097.047l.999.999-1.498.499.499-1.498zm7.698-3.113l-2.42 2.42-.235.18-3.53-3.529.18-.234 7.131-7.131 2.731 2.731-3.69 3.69c-.513.512-.549 1.289-.167 1.873zm6.08-4.959l-4.5 4.5c-.098.099-.226.146-.354.146s-.256-.049-.354-.146c-.195-.194-.195-.512 0-.707l4.5-4.5c.194-.194.512-.194.707 0 .196.195.197.511.001.707zm.107-1.764c-.519-.168-1.108-.062-1.521.35l-.102.102-2.731-2.731.078-.078c.984-.98 2.652-.981 3.608-.023.479.479.743 1.116.743 1.793.001.199-.03.394-.075.587z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPen".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPencil")]
        TiIcon::TiPencil => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 6.879l-3.879-3.879c-.293-.293-.678-.439-1.061-.439-.384 0-.767.146-1.06.439l-10.939 10.939c-.293.293-.558.727-.75 1.188-.192.463-.311.959-.311 1.373v4.5h4.5c.414 0 .908-.119 1.371-.311.463-.192.896-.457 1.189-.75l10.94-10.939c.293-.293.439-.678.439-1.061 0-.384-.146-.767-.439-1.06zm-15.232 8.182l8.293-8.293 1.232 1.232-8.293 8.293-1.232-1.232zm1.732 3.939h-1.5l-1-1v-1.5c0-.077.033-.305.158-.605.01-.02 2.967 2.938 2.967 2.938-.322.134-.548.167-.625.167zm1.439-.768l-1.232-1.232 8.293-8.293 1.232 1.232-8.293 8.293zm9-9l-3.172-3.172 1.293-1.293 3.17 3.172-1.291 1.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPencil".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPhone")]
        TiIcon::TiPhone => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13.374 7.083l3.711-3.71-.438-.434c-.566-.566-1.555-.566-2.121 0l-1.586 1.586c-.284.284-.44.661-.44 1.061s.156.777.438 1.06l.436.437zM6.646 12.939c-.566-.566-1.555-.566-2.121 0l-1.586 1.586c-.283.284-.439.661-.439 1.061s.156.777.441 1.062l.437.432 3.703-3.703-.435-.438zM18.437 4.729l-.354-.354-3.708 3.708.65.649c.095.095.146.22.146.354s-.052.259-.146.354l-5.586 5.586c-.189.188-.518.189-.707 0l-.65-.65-3.702 3.71.354.354c.26.26 1.246 1.105 3.056 1.105 1.616 0 4.256-.712 7.65-4.105 6.773-6.775 3.158-10.55 2.997-10.711z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPhone".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPhoneOutline")]
        TiIcon::TiPhoneOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.502 3.672l-1.795-1.793c-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-1.586 1.586c-1.17 1.17-1.17 3.072 0 4.242l1.379 1.379-4.172 4.172-1.379-1.379c-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-1.586 1.586c-1.17 1.17-1.17 3.072 0 4.242l1.794 1.793c.465.465 1.796 1.545 4.116 1.545 2.764 0 5.694-1.529 8.711-4.545 6.245-6.246 4.825-11.002 3.002-12.828zm-6.209 1.207l1.586-1.586c.195-.195.451-.293.707-.293s.512.098.707.293l1.083 1.082-3.001 3-1.082-1.082c-.391-.391-.391-1.023 0-1.414zm-10 11.414c-.391-.391-.391-1.023 0-1.414l1.586-1.586c.195-.195.451-.293.707-.293s.512.098.707.293l1.082 1.082-2.999 3-1.083-1.082zm11.793-1.207c-3.083 3.082-5.551 3.959-7.297 3.959-1.349 0-2.267-.523-2.702-.959-.004-.004 2.995-3.004 2.995-3.004l.297.297c.195.195.451.293.707.293s.512-.098.707-.293l5.586-5.586c.391-.391.391-1.023 0-1.414l-.297-.297 3.001-3c1.003 1.004 2.467 4.539-2.997 10.004z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPhoneOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPi")]
        TiIcon::TiPi => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.707 8.535c-.391-.391-1.023-.391-1.414 0-1.264 1.264-3.321 1.264-4.586 0-2.045-2.044-5.371-2.042-7.414 0-.391.391-.391 1.023 0 1.414s1.023.391 1.414 0c.374-.374.82-.624 1.293-.776v7.827c0 .553.447 1 1 1s1-.447 1-1v-7.826c.472.152.919.401 1.293.775.768.767 1.715 1.245 2.707 1.437v5.614c0 .553.447 1 1 1s1-.447 1-1v-5.614c.992-.191 1.939-.67 2.707-1.437.391-.39.391-1.023 0-1.414z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPi".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPiOutline")]
        TiIcon::TiPiOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.121 7.121c-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879c-.233.233-.546.362-.879.362s-.646-.129-.879-.362c-1.366-1.366-3.185-2.118-5.121-2.118s-3.755.752-5.121 2.118c-.567.567-.879 1.32-.879 2.121s.312 1.555.879 2.121c.566.567 1.32.879 2.121.879v4.758c0 1.654 1.346 3 3 3s3-1.346 3-3c0 1.654 1.346 3 3 3s3-1.346 3-3v-4.166c.784-.356 1.501-.851 2.12-1.47.568-.567.88-1.321.88-2.122s-.312-1.554-.879-2.121zm-1.414 2.828c-.768.767-1.715 1.245-2.707 1.437v5.614c0 .553-.447 1-1 1s-1-.447-1-1v-5.614c-.992-.191-1.939-.67-2.707-1.437-.374-.374-.821-.623-1.293-.775v7.826c0 .553-.447 1-1 1s-1-.447-1-1v-7.827c-.473.152-.919.402-1.293.776-.195.196-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414 1.021-1.021 2.364-1.532 3.707-1.532s2.685.511 3.707 1.532c.633.632 1.463.948 2.293.948.831 0 1.661-.316 2.293-.948.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.024 0 1.414z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPiOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPin")]
        TiIcon::TiPin => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.729 4.271c-.389-.391-1.021-.393-1.414-.004-.104.104-.176.227-.225.355-.832 1.736-1.748 2.715-2.904 3.293-1.297.64-2.786 1.085-5.186 1.085-.13 0-.26.025-.382.076-.245.102-.439.297-.541.541-.101.244-.101.52 0 .764.051.123.124.234.217.326l3.243 3.243-4.537 6.05 6.05-4.537 3.242 3.242c.092.094.203.166.326.217.122.051.252.078.382.078s.26-.027.382-.078c.245-.102.44-.295.541-.541.051-.121.077-.252.077-.381 0-2.4.444-3.889 1.083-5.166.577-1.156 1.556-2.072 3.293-2.904.129-.049.251-.121.354-.225.389-.393.387-1.025-.004-1.414l-3.997-4.02z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPin".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPinOutline")]
        TiIcon::TiPinOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.436 7.586l-3.998-4.02c-.752-.756-2.063-.764-2.83-.006-.196.196-.35.436-.418.629-.653 1.362-1.354 2.215-2.254 2.727l-.217.105c-.968.485-2.285.979-4.719.979-.266 0-.521.052-.766.152-.484.202-.879.595-1.082 1.084-.199.484-.199 1.041 0 1.525.104.249.25.471.435.651l3.235 3.235-3.822 5.353 5.352-3.822 3.227 3.227c.186.189.406.339.656.441.247.103.503.154.766.154s.519-.052.765-.154c.498-.205.883-.592 1.08-1.078.103-.242.155-.507.155-.768 0-2.436.494-3.752.978-4.721.496-.992 1.369-1.748 2.754-2.414.271-.104.51-.256.711-.457.772-.782.768-2.051-.008-2.822zm-5.248 4.801c-.819 1.643-1.188 3.37-1.195 5.604l-7.993-7.991c2.139 0 3.814-.335 5.396-1.084l.235-.105c1.399-.699 2.468-1.893 3.388-3.834l3.924 4.051c-1.863.893-3.056 1.96-3.755 3.359z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPinOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPipette")]
        TiIcon::TiPipette => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.384 7.331c.073-1.199-.354-2.388-1.146-3.179-.732-.731-1.793-1.152-2.912-1.152-1.176 0-2.206.453-2.825 1.243-.692.883-1.392 2.625-1.769 3.647l-1.616-1.617c-.392-.391-1.023-.391-1.414 0-.392.392-.392 1.023 0 1.414l.293.293-5.231 5.232c-.375.375-.719.912-.968 1.516-.019.043-1.726 4.328-.093 5.959.527.526 1.33.707 2.178.707 1.778-.002 3.753-.787 3.783-.801.602-.248 1.141-.592 1.514-.967l5.232-5.232.293.293c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.022 0-1.414l-1.617-1.616c1.023-.376 2.766-1.075 3.648-1.769.721-.562 1.17-1.493 1.236-2.557zm-16.265 11.944c-.247-.295-.105-1.508.154-2.58l2.422 2.423c-1.071.261-2.283.403-2.576.157zm4.645-1.061c-.188.188-.511.388-.865.533l-.116.042-3.181-3.18.043-.117c.146-.354.346-.678.533-.864l5.232-5.231 3.586 3.586-5.232 5.231z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPipette".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPlane")]
        TiIcon::TiPlane => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.996 13.507l-5.996-3.426v-5.956c0-.827-.673-1.5-1.5-1.5s-1.5.673-1.5 1.5v5.956l-5.996 3.426c-.439.251-.622.79-.426 1.256.197.466.711.713 1.196.573l5.226-1.492v4.451l-1.625 1.3c-.387.31-.488.856-.239 1.284s.776.608 1.235.425l2.129-.852 2.129.852c.121.048.247.071.371.071.347 0 .681-.181.864-.497.249-.428.147-.975-.239-1.284l-1.625-1.3v-4.451l5.226 1.493.274.039c.394 0 .762-.233.922-.612.196-.466.014-1.005-.426-1.256zm-7.496-9.132c-.276 0-.5-.224-.5-.5s.224-.5.5-.5.5.224.5.5-.224.5-.5.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPlane".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPlaneOutline")]
        TiIcon::TiPlaneOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M20.988 12.396l-4.988-2.851v-4.795c0-1.93-1.57-3.5-3.5-3.5s-3.5 1.57-3.5 3.5v4.795l-4.988 2.851c-1.317.752-1.865 2.371-1.276 3.769.589 1.399 2.132 2.135 3.589 1.72l2.675-.765v.838l-.874.699c-1.198.959-1.48 2.667-.653 3.959.827 1.293 2.494 1.753 3.869 1.066.004-.001.5-.183 1.158-.183l1.158.183c1.375.687 3.042.227 3.869-1.066.827-1.292.545-3-.653-3.959l-.874-.699v-.838l2.676.765c1.457.415 3-.321 3.589-1.72s.041-3.017-1.277-3.769zm-.566 2.992c-.197.466-.711.713-1.196.573l-5.226-1.492v4.451l1.625 1.3c.399.319.493.889.218 1.32-.275.43-.828.583-1.29.355-.008-.004-.824-.395-2.053-.395s-2.045.391-2.053.395c-.462.227-1.015.074-1.29-.355-.275-.431-.182-1 .218-1.32l1.625-1.3v-4.451l-5.226 1.493c-.485.14-.999-.107-1.196-.573-.196-.466-.014-1.005.426-1.256l5.996-3.427v-5.956c0-.827.673-1.5 1.5-1.5s1.5.673 1.5 1.5v5.956l5.996 3.426c.44.251.622.79.426 1.256z\" />\n  <circle cx=\"12.5\" cy=\"4.5\" r=\".5\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPlaneOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPlug")]
        TiIcon::TiPlug => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18,6h-1V3c0-0.6-0.4-1-1-1h-2c-0.6,0-1,0.4-1,1v3h-2V3c0-0.6-0.4-1-1-1H8C7.4,2,7,2.4,7,3v3H6C5.4,6,5,6.4,5,7v4\n\tc0,0.1,0,0.1,0,0.2c0.2,2.5,1.8,4.6,4,5.6V20c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2v-3.2c2.2-1,3.7-3.1,4-5.6c0-0.1,0-0.1,0-0.2V7\n\tC19,6.4,18.6,6,18,6z M14,3h2v3h-2V3z M8,3h2v3H8V3z M13,20h-2v-2h2V20z M12,15.5c-2.2,0-4.1-1.5-4.7-3.5h9.5\n\tC16.1,14,14.2,15.5,12,15.5z M17,10.5c0,0.2,0,0.3-0.1,0.5H7.1C7,10.8,7,10.7,7,10.5V8h10V10.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPlug".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPlus")]
        TiIcon::TiPlus => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 10h-4v-4c0-1.104-.896-2-2-2s-2 .896-2 2l.071 4h-4.071c-1.104 0-2 .896-2 2s.896 2 2 2l4.071-.071-.071 4.071c0 1.104.896 2 2 2s2-.896 2-2v-4.071l4 .071c1.104 0 2-.896 2-2s-.896-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPlus".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPlusOutline")]
        TiIcon::TiPlusOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 21c-1.654 0-3-1.346-3-3l.053-3.053-3.035.053c-1.672 0-3.018-1.346-3.018-3s1.346-3 3-3l3.053-.054-.053-2.928c0-1.672 1.346-3.018 3-3.018s3 1.346 3 3l.055 2.946 2.963.054c1.636 0 2.982 1.346 2.982 3s-1.346 3-3 3l-2.945-.053-.055 3.071c0 1.636-1.346 2.982-3 2.982zm-1-8v5.018c0 .533.449.982 1 .982s1-.449 1-1v-5h5.018c.533 0 .982-.449.982-1s-.449-1-1-1h-5v-5c0-.569-.449-1-1-1s-1 .449-1 1v5h-5c-.569 0-1 .449-1 1s.449 1 1 1h5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPlusOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPointOfInterest")]
        TiIcon::TiPointOfInterest => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.5 11c1.93 0 3.5-1.57 3.5-3.5s-1.57-3.5-3.5-3.5-3.5 1.57-3.5 3.5v1.5h-2v-1.5c0-1.93-1.57-3.5-3.5-3.5s-3.5 1.57-3.5 3.5 1.57 3.5 3.5 3.5h1.5v2h-1.5c-1.93 0-3.5 1.57-3.5 3.5s1.57 3.5 3.5 3.5 3.5-1.57 3.5-3.5v-1.5h2v1.5c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5-1.57-3.5-3.5-3.5h-1.5v-2h1.5zm-1.5-3.5c0-.828.673-1.5 1.5-1.5s1.5.672 1.5 1.5c0 .826-.673 1.5-1.5 1.5h-1.5v-1.5zm-6 9c0 .826-.673 1.5-1.5 1.5s-1.5-.674-1.5-1.5c0-.828.673-1.5 1.5-1.5h1.5v1.5zm0-7.5h-1.5c-.827 0-1.5-.674-1.5-1.5 0-.828.673-1.5 1.5-1.5s1.5.672 1.5 1.5v1.5zm4 4h-2v-2h2v2zm3.5 2c.827 0 1.5.672 1.5 1.5 0 .826-.673 1.5-1.5 1.5s-1.5-.674-1.5-1.5v-1.5h1.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiPointOfInterest".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPointOfInterestOutline")]
        TiIcon::TiPointOfInterestOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.5 4c1.93 0 3.5 1.57 3.5 3.5s-1.57 3.5-3.5 3.5h-1.5v2h1.5c1.93 0 3.5 1.57 3.5 3.5s-1.57 3.5-3.5 3.5-3.5-1.57-3.5-3.5v-1.5h-2v1.5c0 1.93-1.57 3.5-3.5 3.5s-3.5-1.57-3.5-3.5 1.57-3.5 3.5-3.5h1.5v-2h-1.5c-1.93 0-3.5-1.57-3.5-3.5s1.57-3.5 3.5-3.5 3.5 1.57 3.5 3.5v1.5h2v-1.5c0-1.93 1.57-3.5 3.5-3.5m-1.5 5h1.5c.827 0 1.5-.674 1.5-1.5 0-.828-.673-1.5-1.5-1.5s-1.5.672-1.5 1.5v1.5m-7.5 0h1.5v-1.5c0-.828-.673-1.5-1.5-1.5s-1.5.672-1.5 1.5c0 .826.673 1.5 1.5 1.5m9 9c.827 0 1.5-.674 1.5-1.5 0-.828-.673-1.5-1.5-1.5h-1.5v1.5c0 .826.673 1.5 1.5 1.5m-9 0c.827 0 1.5-.674 1.5-1.5v-1.5h-1.5c-.827 0-1.5.672-1.5 1.5 0 .826.673 1.5 1.5 1.5m9-16c-1.857 0-3.504.926-4.5 2.341-.996-1.415-2.643-2.341-4.5-2.341-3.033 0-5.5 2.468-5.5 5.5 0 1.857.926 3.504 2.341 4.5-1.415.996-2.341 2.643-2.341 4.5 0 3.032 2.467 5.5 5.5 5.5 1.857 0 3.504-.926 4.5-2.341.996 1.415 2.643 2.341 4.5 2.341 3.033 0 5.5-2.468 5.5-5.5 0-1.857-.926-3.504-2.341-4.5 1.415-.996 2.341-2.643 2.341-4.5 0-3.032-2.467-5.5-5.5-5.5zM13 11h-2v2h2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiPointOfInterestOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPower")]
        TiIcon::TiPower => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.5 18.573c-1.736 0-3.368-.676-4.596-1.903-1.227-1.228-1.904-2.86-1.904-4.597s.677-3.369 1.904-4.597c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-.85.851-1.318 1.981-1.318 3.183s.468 2.333 1.318 3.183c.85.85 1.979 1.317 3.182 1.317s2.332-.468 3.182-1.317c.851-.85 1.318-1.98 1.318-3.183s-.468-2.333-1.318-3.183c-.391-.391-.391-1.023 0-1.414s1.023-.391 1.414 0c1.227 1.229 1.904 2.861 1.904 4.597s-.677 3.369-1.904 4.597c-1.228 1.227-2.86 1.903-4.596 1.903zM11.5 11c-.553 0-1-.448-1-1v-5c0-.552.447-1 1-1s1 .448 1 1v5c0 .552-.447 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPower".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPowerOutline")]
        TiIcon::TiPowerOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.51 6.062c-.814-.815-1.98-1.05-3.01-.729v-.333c0-1.656-1.344-3-3-3s-3 1.344-3 3v.332c-1.029-.319-2.195-.085-3.01.73-1.605 1.606-2.49 3.741-2.49 6.011s.885 4.405 2.49 6.011c1.604 1.605 3.739 2.489 6.01 2.489s4.405-.884 6.01-2.489c1.605-1.605 2.49-3.74 2.49-6.011s-.885-4.405-2.49-6.011zm-7.01-1.062c0-.552.447-1 1-1s1 .448 1 1v5c0 .552-.447 1-1 1s-1-.448-1-1v-5zm-1 3.803v1.197c0 1.104.896 2 2 2s2-.896 2-2v-1.182c.095.284.248.554.475.78.661.661 1.025 1.54 1.025 2.475s-.364 1.814-1.025 2.476c-1.322 1.321-3.627 1.321-4.949 0-.662-.662-1.026-1.541-1.026-2.476s.364-1.814 1.025-2.476c.231-.23.383-.504.475-.794zm6.596 7.867c-1.228 1.228-2.859 1.903-4.596 1.903s-3.368-.676-4.596-1.903c-1.227-1.228-1.904-2.86-1.904-4.597s.677-3.369 1.904-4.597c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-.85.851-1.318 1.981-1.318 3.183s.468 2.333 1.318 3.183c.85.85 1.979 1.317 3.182 1.317s2.332-.468 3.182-1.317c.851-.85 1.318-1.98 1.318-3.183s-.468-2.333-1.318-3.183c-.391-.391-.391-1.023 0-1.414s1.023-.391 1.414 0c1.227 1.229 1.904 2.861 1.904 4.597s-.677 3.369-1.904 4.597z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPowerOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPrinter")]
        TiIcon::TiPrinter => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 5v-2c0-.552-.448-1-1-1h-9c-.552 0-1 .448-1 1v2c-1.654 0-3 1.346-3 3v10c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-10c0-1.654-1.346-3-3-3zm-9-1h7v5h-7v-5zm-2 3v3c0 .552.448 1 1 1h9c.552 0 1-.448 1-1v-3c.551 0 1 .449 1 1v2.5c0 .827-.673 1.5-1.5 1.5h-10c-.827 0-1.5-.673-1.5-1.5v-2.5c0-.551.449-1 1-1zm11 12h-11c-.551 0-1-.449-1-1v-5.513c.419.318.935.513 1.5.513h10c.565 0 1.081-.195 1.5-.513v5.513c0 .551-.449 1-1 1zM13.5 7h-4c-.276 0-.5.224-.5.5s.224.5.5.5h4c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM15 16h-7c-.276 0-.5.224-.5.5s.224.5.5.5h7c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM13.5 5h-4c-.276 0-.5.224-.5.5s.224.5.5.5h4c.276 0 .5-.224.5-.5s-.224-.5-.5-.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPrinter".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPuzzle")]
        TiIcon::TiPuzzle => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.25 11.25c.364 0 .704.145.984.391.549.332.766-.034.766-.391v-1.75c0-.825-.675-1.5-1.5-1.5h-2.75c-.356 0-.724-.216-.391-.766.246-.28.391-.619.391-.984 0-.967-1.007-1.75-2.25-1.75s-2.25.783-2.25 1.75c0 .3.095.576.255.823.507.673.136.927-.255.927h-2.75c-.825 0-1.5.675-1.5 1.5v1.75c0 .391.254.762.928.244.246-.149.522-.244.822-.244.966 0 1.75 1.008 1.75 2.25s-.784 2.25-1.75 2.25c-.364 0-.704-.145-.984-.391-.549-.332-.766.034-.766.391v2.75c0 .825.675 1.5 1.5 1.5h2.75c.391 0 .762-.254.243-.927-.148-.247-.243-.523-.243-.823 0-.967 1.007-1.75 2.25-1.75s2.25.783 2.25 1.75c0 .365-.145.704-.391.984-.333.55.035.766.391.766h2.75c.825 0 1.5-.675 1.5-1.5v-2.75c0-.391-.254-.762-.928-.244-.246.149-.522.244-.822.244-.966 0-1.75-1.008-1.75-2.25s.784-2.25 1.75-2.25z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPuzzle".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiPuzzleOutline")]
        TiIcon::TiPuzzleOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20 11.25v-1.75c0-1.93-1.57-3.5-3.5-3.5h-.759c-.141-1.982-1.953-3.5-4.241-3.5s-4.1 1.518-4.241 3.5h-.759c-1.93 0-3.5 1.57-3.5 3.5v1.75c0 1.012.514 1.847 1.295 2.246-.358.188-.668.469-.894.825-.262.414-.401.908-.401 1.429v2.75c0 1.93 1.57 3.5 3.5 3.5h2.75c.976 0 1.831-.497 2.242-1.299l.036.066c.435.772 1.266 1.233 2.222 1.233h2.75c1.93 0 3.5-1.57 3.5-3.5v-2.75c0-1.013-.515-1.849-1.297-2.247.776-.411 1.297-1.256 1.297-2.253zm-2 7.25c0 .825-.675 1.5-1.5 1.5h-2.75c-.356 0-.724-.216-.391-.766.246-.28.391-.619.391-.984 0-.967-1.007-1.75-2.25-1.75s-2.25.783-2.25 1.75c0 .3.095.576.255.823.507.673.136.927-.255.927h-2.75c-.825 0-1.5-.675-1.5-1.5v-2.75c0-.258.113-.521.384-.521.104 0 .229.039.382.13.28.246.62.391.984.391.966 0 1.75-1.008 1.75-2.25s-.784-2.25-1.75-2.25c-.3 0-.576.095-.822.255-.237.171-.422.243-.562.243-.26 0-.366-.245-.366-.498v-1.75c0-.825.675-1.5 1.5-1.5h2.75c.391 0 .762-.254.243-.927-.148-.247-.243-.523-.243-.823 0-.967 1.007-1.75 2.25-1.75s2.25.783 2.25 1.75c0 .365-.145.704-.391.984-.333.55.035.766.391.766h2.75c.825 0 1.5.675 1.5 1.5v1.75c0 .258-.113.521-.384.521-.104 0-.229-.039-.382-.13-.28-.246-.62-.391-.984-.391-.966 0-1.75 1.008-1.75 2.25s.784 2.25 1.75 2.25c.3 0 .576-.095.822-.255.237-.171.422-.244.562-.243.259 0 .365.245.365.498v2.75zm-13-5.806c.116.032.236.054.365.054.342 0 .683-.119 1.038-.364l.069-.041c.097-.063.188-.093.278-.093.354 0 .75.535.75 1.25s-.396 1.25-.75 1.25c-.108 0-.217-.048-.324-.142l-.143-.104c-.301-.183-.604-.275-.899-.275-.134 0-.261.023-.384.059v-1.594zm12.635 1.558c-.342 0-.683.119-1.038.364l-.069.041c-.097.063-.188.093-.277.093-.354 0-.75-.535-.75-1.25s.396-1.25.75-1.25c.108 0 .217.048.324.142l.143.104c.302.183.604.275.899.275.136 0 .262-.025.384-.062v1.597c-.117-.032-.237-.054-.366-.054zm-6.943 5.748c.101-.346.093-.816-.305-1.396l-.044-.074c-.062-.098-.094-.189-.094-.279 0-.354.534-.75 1.25-.75s1.25.396 1.25.75c0 .108-.048.217-.143.325l-.104.142c-.325.537-.311.979-.22 1.284h-1.59z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiPuzzleOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiRadar")]
        TiIcon::TiRadar => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 20c3.86 0 7-3.141 7-7s-3.14-7-7.003-7c-3.858 0-6.997 3.141-6.997 7s3.14 7 7 7zm-1-11.898v1.898c0 .553.448 1 1 1s1-.447 1-1v-1.898c1.956.398 3.5 1.942 3.899 3.898h-1.899c-.552 0-1 .447-1 1s.448 1 1 1h1.899c-.399 1.956-1.943 3.5-3.899 3.898v-1.898c0-.553-.448-1-1-1s-1 .447-1 1v1.898c-1.956-.398-3.5-1.942-3.899-3.898h1.899c.552 0 1-.447 1-1s-.448-1-1-1h-1.899c.399-1.956 1.942-3.5 3.899-3.898z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiRadar".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiRadarOutline")]
        TiIcon::TiRadarOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.997 4.5c-4.685 0-8.497 3.812-8.497 8.5s3.813 8.5 8.5 8.5c4.688 0 8.5-3.812 8.5-8.5s-3.812-8.5-8.503-8.5zm.003 15c-3.584 0-6.5-2.916-6.5-6.5s2.914-6.5 6.5-6.5c3.584 0 6.5 2.916 6.5 6.5s-2.916 6.5-6.5 6.5zM15.348 12.031l.152-.031h.879c-.383-1.677-1.699-2.995-3.379-3.378v.878c0 .551-.449 1-1 1-.497 0-.892-.371-.969-.846l-.031-.154v-.88c-1.678.382-2.997 1.702-3.38 3.38h.88l.153.031c.476.076.847.472.847.969s-.371.893-.846.969l-.154.031h-.878c.384 1.677 1.702 2.995 3.378 3.379v-.879l.031-.154c.077-.476.472-.846.969-.846s.893.371.969.848l.031.152v.879c1.677-.383 2.996-1.702 3.379-3.379h-.879l-.152-.031c-.477-.076-.848-.472-.848-.969s.371-.893.848-.969zm-.446 2.867c-.264.399-.604.74-1.004 1.002-.256-.81-1.004-1.401-1.897-1.401s-1.642.592-1.898 1.401c-.4-.262-.74-.603-1.003-1.002.81-.256 1.401-1.006 1.401-1.898 0-.895-.592-1.643-1.402-1.898.263-.399.603-.74 1.004-1.002.256.81 1.005 1.401 1.898 1.401.894 0 1.644-.593 1.899-1.403.399.264.74.604 1.002 1.004-.81.256-1.401 1.006-1.401 1.898-.001.893.591 1.643 1.401 1.898z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiRadarOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiRefresh")]
        TiIcon::TiRefresh => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12.872 13.191h5.128v-5.127c-.008-1.135-.671-1.408-1.473-.605l-1.154 1.158c-1.015-.795-2.257-1.23-3.566-1.23-1.55 0-3.009.604-4.104 1.701-1.099 1.092-1.703 2.553-1.703 4.103 0 1.553.604 3.012 1.701 4.107 1.097 1.097 2.555 1.702 4.106 1.702 1.55 0 3.009-.605 4.106-1.703.296-.297.558-.621.78-.965.347-.541.19-1.26-.35-1.605-.539-.346-1.258-.189-1.604.35-.133.207-.292.4-.468.58-.659.658-1.534 1.02-2.464 1.02-.93 0-1.805-.361-2.464-1.02-.657-.658-1.02-1.533-1.02-2.465 0-.93.362-1.805 1.02-2.461.659-.658 1.534-1.021 2.464-1.021.688 0 1.346.201 1.909.572l-1.448 1.451c-.803.802-.53 1.458.604 1.458z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiRefresh".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiRefreshOutline")]
        TiIcon::TiRefreshOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.368 4.998c-.488 0-1.2.145-1.956.773-1.036-.489-2.189-.771-3.412-.771-4.418 0-8 3.582-8 8s3.582 8 8 8c4.312 0 8-3.316 8-8v-4.936c-.016-2.111-1.375-3.066-2.632-3.066zm.632 8.002h-5.128c-1.134 0-1.407-.561-.604-1.363l1.448-1.402c-.562-.371-1.22-.549-1.909-.549-.93 0-1.805.375-2.464 1.033-.657.656-1.02 1.537-1.02 2.467 0 .933.362 1.811 1.02 2.469.659.658 1.534 1.021 2.464 1.021s1.805-.36 2.465-1.019c.177-.18.334-.372.468-.579.222-.345.596-.533.979-.533.216 0 .433.06.625.185.54.346.697 1.063.351 1.604-.223.344-.484.668-.78.965-1.097 1.099-2.556 1.703-4.106 1.703-1.55 0-3.009-.604-4.104-1.701-1.097-1.096-1.701-2.555-1.701-4.106 0-1.551.604-3.012 1.702-4.104 1.096-1.098 2.554-1.7 4.104-1.7 1.311 0 2.551.436 3.566 1.229l1.154-1.158c.311-.312.602-.461.841-.461.377 0 .627.372.632 1.065v4.934zm-7.08.05c.162.392.63.95 1.952.95h1.299s-.21.504-.614.907-1.086.745-1.75.745-1.289-.246-1.758-.715c-.468-.47-.727-1.088-.727-1.752s.258-1.139.726-1.604c.472-.472 1.097-.581 1.759-.581l-.246.123c-.935.934-.803 1.536-.641 1.927z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiRefreshOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiRss")]
        TiIcon::TiRss => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M6.002 15.999c-1.107 0-2.004.897-2.002 2.001 0 1.104.896 2.001 2.002 1.999 1.103.002 2-.894 1.998-1.999.002-1.107-.895-2.003-1.998-2.001zM6 4c-1.104 0-2 .896-2 2s.896 2 2 2c5.514 0 10 4.486 10 10 0 1.104.896 2 2 2s2-.896 2-2c0-7.72-6.28-14-14-14zM6 10c-1.104 0-2 .896-2 2s.896 2 2 2c2.205 0 4 1.794 4 4 0 1.104.896 2 2 2s2-.896 2-2c0-4.411-3.589-8-8-8z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiRss".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiRssOutline")]
        TiIcon::TiRssOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 4.999c-1.657 0-3.011 1.344-3.011 3l.005 9c0 2.209 1.793 4 4.002 4l9.003.001c1.655 0 3-1.346 3-3.001.001-7.179-5.819-13-12.999-13zm1.001 14c-1.105.002-2.001-.894-2.001-1.999-.002-1.105.894-2.001 2.001-2.001 1.105-.002 2.001.894 1.999 2.001.002 1.105-.894 2.001-1.999 1.999zm4.499 0c-.829 0-1.5-.671-1.5-1.5 0-1.931-1.57-3.5-3.5-3.5-.829 0-1.5-.671-1.5-1.5s.671-1.5 1.5-1.5c3.584 0 6.5 2.916 6.5 6.5 0 .829-.671 1.5-1.5 1.5zm4 0c-.829 0-1.5-.671-1.5-1.5 0-4.136-3.364-7.5-7.5-7.5-.829 0-1.5-.671-1.5-1.5s.671-1.5 1.5-1.5c5.79 0 10.5 4.71 10.5 10.5 0 .829-.671 1.5-1.5 1.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiRssOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiScissors")]
        TiIcon::TiScissors => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.625 5.515c-1-1.522-2.915-1.67-4.397-.824l-.186.107-.076-.003c-1.042 0-2.01.511-2.604 1.369l-.034.045c-.43.645-.723 1.236-1.005 1.809-.255.516-.5 1.01-.824 1.483-.325-.475-.57-.97-.826-1.486-.283-.571-.575-1.162-1.004-1.806l-.033-.044c-.593-.859-1.562-1.37-2.603-1.37-1.747 0-3.167 1.42-3.167 3.166 0 1.747 1.421 3.168 3.167 3.168.775 0 1.515-.287 2.087-.791l.652 1.198c-1.621 1.876-2.979 4.054-3.019 4.121-1.236 1.702.705 4.42.789 4.534.094.131.245.207.405.207.204-.012.357-.11.439-.261l3.112-5.717 3.113 5.717c.082.15.235.249.407.26.174.019.336-.066.437-.206.083-.114 2.024-2.832.809-4.504l-.323-.521c-1.076-1.736-1.187-1.916-2.715-3.634l.651-1.195c.572.504 1.313.791 2.088.791 1.746 0 3.167-1.421 3.167-3.168 0-.634-.191-1.246-.547-1.768.472-.27.997-.123 1.456.095.466.191.897-.377.584-.772zm-13.625 3.485c-.552 0-1-.447-1-1s.448-1 1-1 1 .447 1 1-.448 1-1 1zm4.5 3.395c-.277 0-.5-.225-.5-.5 0-.277.223-.5.5-.5s.5.223.5.5c0 .275-.223.5-.5.5zm4.5-3.395c-.552 0-1-.447-1-1s.448-1 1-1 1 .447 1 1-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiScissors".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiScissorsOutline")]
        TiIcon::TiScissorsOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.124 5.27l.25.013c.163.022.319.02.468.077.146.05.292.084.42.146.254.131.471.262.631.422.174.137.248.279.321.371l.108.146-.166-.074c-.105-.043-.258-.139-.442-.16-.129-.037-.282-.047-.442-.047l-.15.002-.318.061c-.103.01-.201.078-.295.113-.186.088-.35.203-.461.34l-.057.076c.451.477.732 1.115.732 1.824 0 1.471-1.195 2.668-2.666 2.668-.908 0-1.707-.457-2.189-1.15l-1.657 2.625c1.712 1.92 3.22 4.348 3.22 4.348 1.037 1.429-.789 3.947-.789 3.947l-3.552-6.522-3.551 6.522s-1.826-2.52-.789-3.947c0 0 1.507-2.428 3.22-4.348l-1.656-2.625c-.482.693-1.283 1.15-2.188 1.15-1.472 0-2.667-1.197-2.667-2.668 0-1.469 1.195-2.666 2.667-2.666.925 0 1.739.475 2.218 1.193.955 1.428 1.739 3.156 2.748 4.334 1.008-1.178 1.792-2.906 2.746-4.336.48-.717 1.295-1.191 2.221-1.191l.186.018.326-.188.383-.211c.132-.072.297-.107.449-.158.224-.07.475-.105.721-.105m-2.069 4.376c.273 0 .547-.104.756-.312.416-.416.416-1.092 0-1.508-.209-.208-.481-.312-.754-.312s-.545.104-.754.312l-.24.377c-.144.381-.066.826.24 1.131.207.209.479.312.752.312m-9.931 0c.272 0 .545-.104.752-.312.308-.305.382-.75.237-1.131l-.237-.377c-.207-.208-.48-.312-.753-.312s-.547.104-.755.312c-.417.416-.417 1.092 0 1.508.208.208.482.312.756.312m4.965 3.762c.218 0 .396-.178.396-.395 0-.22-.176-.396-.396-.396-.219 0-.394.176-.394.396 0 .217.176.395.394.395m7.035-10.138c-.448 0-.901.066-1.312.191l-.117.036c-.168.051-.426.126-.707.28l-.271.148c-1.388.102-2.656.815-3.467 1.961l-.079.108c-.39.584-.741 1.192-1.082 1.784-.339-.592-.69-1.199-1.081-1.784l-.078-.107c-.877-1.239-2.289-1.974-3.807-1.974-2.574 0-4.667 2.093-4.667 4.666s2.094 4.668 4.667 4.668c.309 0 .611-.03.908-.09-1.016 1.338-1.777 2.53-1.948 2.803-1.714 2.467.392 5.619.835 6.229.377.521.98.826 1.619.826l.129-.004c.686-.044 1.3-.437 1.628-1.039l1.795-3.298 1.795 3.298c.328.604.943.995 1.628 1.039l.129.004c.639 0 1.241-.306 1.619-.826.443-.61 2.549-3.764.834-6.229-.17-.271-.932-1.465-1.947-2.803.295.06.601.09.908.09 2.573 0 4.668-2.095 4.668-4.668l-.004-.179c.467-.096.898-.356 1.2-.758.547-.729.536-1.733-.032-2.445l-.049-.067-.039-.053c-.102-.146-.279-.394-.572-.644-.35-.326-.738-.547-1.045-.705-.275-.136-.488-.201-.628-.244l-.062-.02c-.333-.117-.622-.146-.784-.16l-.054-.006-.086-.01-.444-.018z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiScissorsOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiShoppingBag")]
        TiIcon::TiShoppingBag => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 4h-10c-1.654 0-3 1.346-3 3v11c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-11c0-1.654-1.346-3-3-3zm1 14c0 .551-.448 1-1 1h-10c-.552 0-1-.449-1-1v-7.28c.296.174.635.28 1 .28h1.5c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5h1.5c.365 0 .704-.106 1-.279v7.279zm-8.5-7h5c0 1.378-1.121 2.5-2.5 2.5s-2.5-1.122-2.5-2.5zm8.5-2c0 .551-.448 1-1 1h-10c-.552 0-1-.449-1-1v-2c0-.551.448-1 1-1h10c.552 0 1 .449 1 1v2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiShoppingBag".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiShoppingCart")]
        TiIcon::TiShoppingCart => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M20.756 5.345c-.191-.219-.466-.345-.756-.345h-13.819l-.195-1.164c-.08-.482-.497-.836-.986-.836h-2.25c-.553 0-1 .447-1 1s.447 1 1 1h1.403l1.86 11.164.045.124.054.151.12.179.095.112.193.13.112.065c.116.047.238.075.367.075h11.001c.553 0 1-.447 1-1s-.447-1-1-1h-10.153l-.166-1h11.319c.498 0 .92-.366.99-.858l1-7c.041-.288-.045-.579-.234-.797zm-1.909 1.655l-.285 2h-3.562v-2h3.847zm-4.847 0v2h-3v-2h3zm0 3v2h-3v-2h3zm-4-3v2h-3l-.148.03-.338-2.03h3.486zm-2.986 3h2.986v2h-2.653l-.333-2zm7.986 2v-2h3.418l-.285 2h-3.133z\" />\n  <circle cx=\"8.5\" cy=\"19.5\" r=\"1.5\" />\n  <circle cx=\"17.5\" cy=\"19.5\" r=\"1.5\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiShoppingCart".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialAtCircular")]
        TiIcon::TiSocialAtCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M11.844 7.5c-2.481 0-4.438 2.019-4.438 4.5s2.05 4.5 4.531 4.5c.908 0 1.799-.27 2.547-.778.228-.155.295-.466.139-.694-.155-.229-.462-.287-.691-.132-.58.396-1.258.604-1.965.604-1.93 0-3.499-1.57-3.499-3.5s1.446-3.5 3.376-3.5 3.375 1.57 3.375 3.5v.25c0 .414-.336.75-.75.75s-.75-.336-.75-.75v-1.75c0-.276-.099-.5-.375-.5-.205 0-.318.124-.396.301-.303-.188-.628-.301-1.01-.301-1.104 0-1.984.896-1.984 2s.904 2 2.008 2c.562 0 1.073-.235 1.438-.609.319.369.664.609 1.192.609.965 0 1.627-.785 1.627-1.75v-.25c0-2.481-1.894-4.5-4.375-4.5zm.125 5.5c-.551 0-1-.449-1-1s.449-1 1-1 1 .449 1 1-.449 1-1 1zM12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialAtCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialDribbble")]
        TiIcon::TiSocialDribbble => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 3c-4.962 0-9 4.037-9 9s4.038 9 9 9 9-4.037 9-9-4.038-9-9-9zm6.962 8.275c-1.765-.289-3.534-.187-5.205.262-.18-.436-.383-.859-.59-1.283 1.422-.81 2.685-1.912 3.713-3.262 1.143 1.113 1.909 2.611 2.082 4.283zm-2.832-4.914c-.939 1.243-2.1 2.259-3.401 3.009-.782-1.445-1.729-2.8-2.807-4.056.657-.204 1.355-.314 2.078-.314 1.545 0 2.971.51 4.13 1.361zm-7.183-.65c1.119 1.265 2.093 2.645 2.892 4.117-2.061.957-4.396 1.294-6.717.899.408-2.212 1.86-4.058 3.825-5.016zm-3.947 6.289l.015-.294c.676.111 1.353.188 2.024.188 1.827 0 3.607-.426 5.237-1.187.182.373.365.744.525 1.127-2.429.866-4.583 2.486-6.101 4.726-1.056-1.227-1.7-2.818-1.7-4.56zm2.393 5.257c1.43-2.129 3.465-3.673 5.764-4.487.683 1.854 1.123 3.795 1.292 5.779-.763.287-1.587.451-2.449.451-1.765 0-3.375-.661-4.607-1.743zm8.014.852c-.196-1.932-.631-3.822-1.293-5.632 1.564-.404 3.222-.486 4.871-.19-.102 2.502-1.516 4.668-3.578 5.822z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialDribbble".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialDribbbleCircular")]
        TiIcon::TiSocialDribbbleCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 21c-4.962 0-9-4.037-9-9s4.038-9 9-9 9 4.037 9 9-4.038 9-9 9zm0-16c-3.86 0-7 3.141-7 7s3.14 7 7 7 7-3.141 7-7-3.14-7-7-7zM12 6.5c-3.033 0-5.5 2.468-5.5 5.5s2.467 5.5 5.5 5.5 5.5-2.468 5.5-5.5-2.467-5.5-5.5-5.5zm4.49 5.402c-1.048-.186-2.1-.18-3.103.042-.127-.326-.267-.647-.417-.965.875-.524 1.652-1.221 2.284-2.07.746.785 1.211 1.834 1.236 2.993zm-2-3.646c-.546.748-1.215 1.367-1.975 1.832-.479-.856-1.046-1.663-1.692-2.412.378-.103.767-.176 1.177-.176.921 0 1.776.28 2.49.756zm-4.641-.184c.687.758 1.278 1.59 1.776 2.473-1.238.531-2.622.691-3.998.437.293-1.259 1.118-2.302 2.222-2.91zm-2.349 3.928c.468.064.936.121 1.399.121 1.106 0 2.187-.244 3.185-.683.123.261.238.524.344.793-1.469.526-2.769 1.489-3.728 2.805-.738-.802-1.2-1.862-1.2-3.036zm1.948 3.699c.842-1.189 2.004-2.057 3.318-2.527.314 1.001.518 2.039.596 3.095-.433.138-.884.233-1.362.233-.948 0-1.826-.298-2.552-.801zm4.872.137c-.099-1-.296-1.983-.593-2.937.87-.176 1.778-.172 2.683.001-.256 1.247-1.035 2.296-2.09 2.936z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialDribbbleCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialFacebook")]
        TiIcon::TiSocialFacebook => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 10h3v3h-3v7h-3v-7h-3v-3h3v-1.255c0-1.189.374-2.691 1.118-3.512.744-.823 1.673-1.233 2.786-1.233h2.096v3h-2.1c-.498 0-.9.402-.9.899v2.101z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialFacebook".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialFacebookCircular")]
        TiIcon::TiSocialFacebookCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.354 5.624c-1.75-1.741-3.888-2.624-6.354-2.624-2.489 0-4.633.884-6.373 2.625-1.743 1.741-2.627 3.887-2.627 6.375 0 2.465.883 4.603 2.624 6.354 1.741 1.756 3.886 2.646 6.376 2.646 2.467 0 4.605-.89 6.356-2.643 1.755-1.753 2.644-3.892 2.644-6.357 0-2.488-.89-4.634-2.646-6.376zm-1.412 11.319c-1.137 1.139-2.436 1.788-3.942 1.985v-4.928h2v-2h-2v-1.4c0-.331.269-.6.601-.6h1.399v-2h-1.397c-.742 0-1.361.273-1.857.822-.496.547-.746 1.215-.746 2.008v1.17h-2v2h2v4.93c-1.522-.195-2.826-.845-3.957-1.984-1.375-1.384-2.043-3.002-2.043-4.946 0-1.966.667-3.588 2.042-4.96 1.37-1.373 2.992-2.04 4.958-2.04 1.945 0 3.562.668 4.945 2.043 1.383 1.372 2.055 2.994 2.055 4.957 0 1.941-.673 3.559-2.058 4.943z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialFacebookCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialFlickr")]
        TiIcon::TiSocialFlickr => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M7.5 16c-2.206 0-4-1.794-4-4s1.794-4 4-4 4 1.794 4 4-1.794 4-4 4zm0-6c-1.103 0-2 .897-2 2s.897 2 2 2 2-.897 2-2-.897-2-2-2zM16.5 8c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialFlickr".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialFlickrCircular")]
        TiIcon::TiSocialFlickrCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 21c-2.489 0-4.635-.89-6.376-2.646-1.741-1.751-2.624-3.889-2.624-6.354 0-2.488.884-4.634 2.627-6.375 1.74-1.741 3.885-2.625 6.373-2.625 2.466 0 4.604.883 6.354 2.624 1.755 1.742 2.646 3.888 2.646 6.376 0 2.465-.89 4.604-2.644 6.357-1.751 1.753-3.889 2.643-6.356 2.643zm0-16c-1.966 0-3.588.667-4.958 2.04-1.374 1.372-2.042 2.994-2.042 4.96 0 1.944.668 3.562 2.043 4.945 1.372 1.383 2.993 2.055 4.957 2.055 1.943 0 3.56-.673 4.941-2.057 1.386-1.384 2.059-3.002 2.059-4.943 0-1.963-.672-3.585-2.055-4.957-1.383-1.375-3-2.043-4.945-2.043zM9 14.5c-1.379 0-2.5-1.121-2.5-2.5s1.121-2.5 2.5-2.5 2.5 1.121 2.5 2.5-1.121 2.5-2.5 2.5zm0-4c-.827 0-1.5.673-1.5 1.5s.673 1.5 1.5 1.5 1.5-.673 1.5-1.5-.673-1.5-1.5-1.5zM15 14.5c-1.379 0-2.5-1.121-2.5-2.5s1.121-2.5 2.5-2.5 2.5 1.121 2.5 2.5-1.121 2.5-2.5 2.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialFlickrCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialGithub")]
        TiIcon::TiSocialGithub => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14.435 12.973c.269 0 .492.133.686.396.192.265.294.588.294.975 0 .385-.102.711-.294.973-.193.265-.417.396-.686.396-.278 0-.522-.131-.715-.396-.192-.262-.294-.588-.294-.973 0-.387.102-.71.294-.975.192-.264.436-.396.715-.396m3.44-3.559c.746.811 1.125 1.795 1.125 2.953 0 .748-.086 1.423-.259 2.023-.175.597-.394 1.084-.654 1.459-.264.376-.588.705-.974.989-.386.286-.741.492-1.065.623-.325.132-.695.233-1.111.306-.417.071-.726.111-.943.123l-.685.014-.547.015c-.301.013-.56.016-.762.016s-.461-.003-.762-.016l-.547-.015-.685-.014c-.218-.012-.526-.052-.943-.123-.423-.072-.786-.174-1.111-.306-.324-.131-.68-.337-1.064-.623-.387-.284-.711-.613-.975-.989-.261-.375-.479-.862-.654-1.459-.173-.6-.259-1.275-.259-2.023 0-1.158.379-2.143 1.125-2.953-.082-.041-.085-.447-.008-1.217.063-.771.227-1.482.495-2.132.934.099 2.09.629 3.471 1.581.466-.119 1.101-.183 1.917-.183.852 0 1.491.064 1.918.184.629-.425 1.23-.771 1.805-1.034.584-.261 1.005-.416 1.269-.457l.396-.09c.27.649.434 1.36.496 2.132.076.769.073 1.175-.009 1.216m-5.845 7.82c1.688 0 2.954-.202 3.821-.607.855-.404 1.292-1.238 1.292-2.496 0-.73-.273-1.34-.822-1.828-.278-.263-.613-.425-.989-.486-.375-.061-.949-.061-1.72 0-.769.062-1.298.09-1.582.09-.385 0-.8-.018-1.319-.059-.52-.04-.928-.065-1.223-.078-.294-.009-.609.027-.958.108-.345.082-.629.224-.853.425-.521.469-.79 1.077-.79 1.828 0 1.258.426 2.092 1.28 2.496.85.405 2.113.607 3.802.607h.061m-2.434-4.261c.268 0 .492.133.685.396.192.265.294.588.294.975 0 .385-.102.711-.294.973-.192.265-.417.396-.685.396-.279 0-.522-.131-.716-.396-.192-.262-.294-.588-.294-.973 0-.387.102-.71.294-.975.193-.264.436-.396.716-.396\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialGithub".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialGithubCircular")]
        TiIcon::TiSocialGithubCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 21c-4.963 0-9-4.038-9-9s4.037-9 9-9 9 4.038 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.14-7 7s3.141 7 7 7 7-3.14 7-7-3.141-7-7-7zM13.565 12.626c.171 0 .316.084.441.255.124.169.187.378.187.625 0 .248-.062.457-.187.626-.125.169-.271.254-.441.254-.181 0-.337-.084-.461-.254-.124-.169-.187-.378-.187-.626s.062-.456.187-.625c.125-.171.281-.255.461-.255m2.21-2.289c.482.522.725 1.155.725 1.898 0 .482-.057.915-.166 1.301-.111.384-.252.698-.42.939-.171.242-.378.454-.627.635-.249.184-.478.316-.685.401-.208.085-.446.15-.716.196-.266.047-.467.072-.606.079l-.44.009-.352.01-.488.011-.488-.011-.352-.01-.44-.009c-.14-.007-.341-.032-.606-.079-.271-.045-.508-.11-.716-.196-.207-.084-.436-.217-.684-.401-.25-.182-.457-.394-.628-.635-.168-.241-.309-.555-.42-.939-.109-.386-.166-.819-.166-1.301 0-.743.242-1.376.725-1.898-.053-.026-.056-.286-.008-.782.043-.496.148-.953.319-1.37.602.064 1.343.404 2.23 1.017.3-.078.71-.118 1.233-.118.549 0 .959.04 1.234.118.404-.273.791-.496 1.16-.666.374-.168.644-.267.814-.293l.254-.058c.172.417.277.875.32 1.37.05.496.047.756-.006.782m-3.754 5.027c1.083 0 1.899-.129 2.454-.39.553-.26.833-.796.833-1.605 0-.469-.176-.861-.529-1.174-.181-.17-.394-.273-.638-.313-.238-.039-.607-.039-1.104 0-.495.04-.834.058-1.016.058-.248 0-.517-.013-.851-.039l-.783-.049c-.191-.006-.395.018-.616.069-.223.053-.404.143-.55.273-.336.3-.507.691-.507 1.174 0 .809.274 1.345.821 1.605.547.261 1.361.39 2.444.39m-1.524-2.737c.17 0 .316.084.44.255.124.169.187.378.187.625 0 .248-.062.457-.187.626-.124.169-.271.254-.44.254-.182 0-.337-.084-.462-.254-.124-.169-.187-.378-.187-.626s.062-.456.187-.625c.125-.171.28-.255.462-.255\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialGithubCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialGooglePlus")]
        TiIcon::TiSocialGooglePlus => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12.9 13.5l-.7-.5c-.2-.2-.5-.4-.5-.8s.3-.7.6-1c.8-.6 1.7-1.3 1.7-2.8 0-1.5-.9-2.3-1.4-2.7h1.2l1.2-.7h-4.1c-1 0-2.4.2-3.5 1.1-.8.7-1.2 1.7-1.2 2.6 0 1.5 1.2 3.1 3.3 3.1h.6c-.1.2-.2.4-.2.7 0 .6.3 1 .6 1.3-.9.1-2.5.2-3.8.9-1.2.7-1.5 1.7-1.5 2.4 0 1.5 1.4 2.8 4.2 2.8 3.4 0 5.2-1.9 5.2-3.7 0-1.3-.8-1.9-1.7-2.7zm-2.5-2.2c-1.7 0-2.5-2.2-2.5-3.5 0-.5.1-1 .4-1.5.3-.4.9-.7 1.4-.7 1.6 0 2.5 2.2 2.5 3.6 0 .4 0 1-.5 1.4-.3.4-.9.7-1.3.7zm0 7.9c-2.1 0-3.5-1-3.5-2.4s1.3-1.9 1.7-2c.8-.3 1.9-.3 2.1-.3h.5c1.5 1.1 2.1 1.6 2.1 2.6 0 1.2-1 2.1-2.9 2.1zM17 12h-2v-1h2v-1.9l1-.1v2h2v1h-2v2h-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialGooglePlus".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialGooglePlusCircular")]
        TiIcon::TiSocialGooglePlusCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12.8 13.1l-.4-.3c-.1-.1-.3-.2-.3-.5l.3-.6c.5-.4 1-.8 1-1.7s-.6-1.4-.8-1.6h.7l.7-.4h-2.4c-.6 0-1.4.1-2.1.6-.5.4-.8 1-.8 1.6 0 .9.7 1.9 2 1.9h.4c-.1.1-.1.2-.1.4 0 .4.2.6.4.8-.5 0-1.5.1-2.3.5-.7.4-.9 1-.9 1.4 0 .9.8 1.7 2.5 1.7 2 0 3.1-1.1 3.1-2.2 0-.7-.5-1.1-1-1.6zm-1.6-1.3c-1 0-1.5-1.3-1.5-2.1.1-.4.1-.7.3-.9s.5-.4.8-.4c1 0 1.5 1.3 1.5 2.2 0 .2 0 .6-.3.9-.1.1-.5.3-.8.3zm.1 4.7c-1.3 0-2.1-.6-2.1-1.4 0-.8.8-1.1 1-1.2.5-.2 1.1-.2 1.2-.2h.3c.9.6 1.3 1 1.3 1.6 0 .7-.6 1.2-1.7 1.2zM15 12h-1v-1h1v-1h1v1h1v1h-1v1h-1zM12 21c-5 0-9-4-9-9s4-9 9-9 9 4 9 9-4 9-9 9zm0-16c-3.9 0-7 3.1-7 7s3.1 7 7 7 7-3.1 7-7-3.1-7-7-7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialGooglePlusCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialInstagram")]
        TiIcon::TiSocialInstagram => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 3h-12c-1.7 0-3 1.3-3 3v12c0 1.7 1.3 3 3 3h12c1.7 0 3-1.3 3-3v-12c0-1.7-1.3-3-3-3zm-6 6c1.7 0 3 1.3 3 3s-1.3 3-3 3-3-1.3-3-3 1.3-3 3-3zm3.8-2c0-.7.6-1.2 1.2-1.2s1.2.6 1.2 1.2-.5 1.2-1.2 1.2-1.2-.5-1.2-1.2zm2.2 12h-12c-.6 0-1-.4-1-1v-6h2c0 2.8 2.2 5 5 5s5-2.2 5-5h2v6c0 .6-.4 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialInstagram".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialInstagramCircular")]
        TiIcon::TiSocialInstagramCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 3c-5 0-9 4-9 9s4 9 9 9 9-4 9-9-4-9-9-9zm0 7c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm2.8-2c0-.7.6-1.2 1.2-1.2s1.2.6 1.2 1.2-.5 1.2-1.2 1.2-1.2-.5-1.2-1.2zm-2.8 11c-3.9 0-7-3.1-7-7h3c0 2.2 1.8 4 4 4s4-1.8 4-4h3c0 3.9-3.1 7-7 7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialInstagramCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialLastFm")]
        TiIcon::TiSocialLastFm => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15.942 16.182c2.374 0 3.558-.791 3.558-2.373 0-1.304-.749-2.132-2.254-2.49l-1.119-.235c-.637-.159-.951-.495-.951-1.009 0-.594.396-.889 1.186-.889.869 0 1.323.334 1.363 1.006l1.717-.178c-.114-1.463-1.109-2.195-2.962-2.195-2.019 0-3.026.832-3.026 2.495 0 1.182.654 1.935 1.958 2.251l1.188.236c.79.196 1.186.555 1.186 1.068 0 .631-.614.949-1.842.949-1.498 0-2.489-.732-2.962-2.195l-.597-1.721c-.354-1.145-.796-1.947-1.334-2.401-.53-.45-1.367-.683-2.519-.683-1.069 0-2.007.396-2.815 1.188-.811.791-1.217 1.838-1.217 3.142 0 1.223.383 2.203 1.156 2.935.774.733 1.688 1.099 2.756 1.099 1.069 0 1.918-.256 2.55-.77l-.53-1.485c-.554.556-1.211.833-1.96.833-.63 0-1.175-.248-1.628-.744-.455-.492-.686-1.137-.686-1.927 0-.989.247-1.708.743-2.163.497-.455 1.056-.681 1.689-.681.674 0 1.155.177 1.457.53.296.357.56.912.797 1.662l.537 1.721c.632 2.014 2.154 3.024 4.561 3.024\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialLastFm".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialLastFmCircular")]
        TiIcon::TiSocialLastFmCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 21c-2.489 0-4.635-.89-6.376-2.646-1.741-1.751-2.624-3.89-2.624-6.354 0-2.489.884-4.633 2.627-6.375 1.74-1.741 3.885-2.625 6.373-2.625 2.466 0 4.604.883 6.354 2.624 1.755 1.742 2.646 3.887 2.646 6.376 0 2.464-.89 4.604-2.644 6.357-1.751 1.754-3.889 2.643-6.356 2.643zm0-16c-1.966 0-3.588.667-4.958 2.04-1.374 1.372-2.042 2.994-2.042 4.96 0 1.944.668 3.562 2.043 4.945 1.372 1.383 2.993 2.055 4.957 2.055 1.943 0 3.56-.673 4.941-2.056 1.386-1.385 2.059-3.002 2.059-4.944 0-1.963-.672-3.585-2.055-4.957-1.383-1.375-3-2.043-4.945-2.043zM14.199 14.333c1.335 0 2-.444 2-1.333 0-.733-.422-1.199-1.267-1.4l-.632-.133c-.354-.089-.534-.277-.534-.566 0-.334.224-.5.666-.5.49 0 .746.188.767.565l.967-.1c-.063-.822-.622-1.233-1.666-1.233-1.134 0-1.699.467-1.699 1.401 0 .665.365 1.088 1.099 1.267l.668.133c.443.11.666.312.666.601 0 .354-.345.532-1.034.532-.844 0-1.398-.411-1.666-1.233l-.334-.967c-.199-.644-.449-1.095-.75-1.35-.3-.255-.771-.384-1.416-.384-.601 0-1.128.223-1.584.667-.456.445-.683 1.033-.683 1.767 0 .688.216 1.239.649 1.649.435.413.95.617 1.55.617.602 0 1.078-.144 1.434-.433l-.299-.834c-.311.312-.679.468-1.101.468-.354 0-.662-.14-.916-.417-.257-.277-.385-.64-.385-1.084 0-.556.139-.961.417-1.217s.594-.383.951-.383c.379 0 .648.1.816.299.167.201.315.512.45.935l.3.967c.356 1.133 1.212 1.699 2.566 1.699\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialLastFmCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialLinkedin")]
        TiIcon::TiSocialLinkedin => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 19h-3v-10h3v10zm11 0h-3v-5.342c0-1.392-.496-2.085-1.479-2.085-.779 0-1.273.388-1.521 1.165v6.262h-3s.04-9 0-10h2.368l.183 2h.062c.615-1 1.598-1.678 2.946-1.678 1.025 0 1.854.285 2.487 1.001.637.717.954 1.679.954 3.03v5.647z\" />\n<ellipse cx=\"6.5\" cy=\"6.5\" rx=\"1.55\" ry=\"1.5\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialLinkedin".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialLinkedinCircular")]
        TiIcon::TiSocialLinkedinCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M10.033 15.3h-1.6v-5.199h1.6v5.199zm-.8-5.866c-.577 0-.866-.267-.866-.8 0-.223.082-.412.25-.567.166-.155.371-.233.616-.233.577 0 .866.268.866.801s-.288.799-.866.799zm6.734 5.866h-1.633v-2.9c0-.755-.268-1.133-.801-1.133-.422 0-.699.211-.834.633-.043.067-.066.201-.066.4v3h-1.633v-3.533c0-.8-.012-1.355-.033-1.666h1.4l.1.699c.367-.556.9-.833 1.633-.833.557 0 1.006.194 1.35.583.346.389.518.95.518 1.684v3.066zM12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialLinkedinCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialPinterest")]
        TiIcon::TiSocialPinterest => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12.486 4.771c-4.23 0-6.363 3.033-6.363 5.562 0 1.533.581 2.894 1.823 3.401.205.084.387.004.446-.221l.182-.717c.061-.221.037-.3-.127-.495-.359-.422-.588-.972-.588-1.747 0-2.25 1.683-4.264 4.384-4.264 2.392 0 3.706 1.463 3.706 3.412 0 2.568-1.137 4.734-2.824 4.734-.932 0-1.629-.77-1.405-1.715.268-1.13.786-2.347.786-3.16 0-.729-.392-1.336-1.2-1.336-.952 0-1.718.984-1.718 2.304 0 .841.286 1.409.286 1.409l-1.146 4.852c-.34 1.44-.051 3.206-.025 3.385.013.104.149.131.21.051.088-.115 1.223-1.517 1.607-2.915.111-.396.627-2.445.627-2.445.311.589 1.213 1.108 2.175 1.108 2.863 0 4.804-2.608 4.804-6.103-.003-2.64-2.24-5.1-5.64-5.1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialPinterest".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialPinterestCircular")]
        TiIcon::TiSocialPinterestCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7zM12.335 8c-2.468 0-3.712 1.77-3.712 3.244 0 .895.338 1.688 1.063 1.984.119.049.226.002.261-.129l.105-.418c.035-.129.021-.175-.074-.289-.209-.246-.344-.566-.344-1.019 0-1.312.982-2.487 2.558-2.487 1.396 0 2.161.853 2.161 1.99 0 1.498-.662 2.762-1.646 2.762-.543 0-.95-.449-.82-1.001.156-.658.459-1.368.459-1.843 0-.426-.229-.779-.7-.779-.556 0-1.002.574-1.002 1.344 0 .49.166.822.166.822l-.669 2.83c-.198.84-.029 1.87-.015 1.974.008.062.087.077.123.03.052-.067.713-.885.938-1.7.064-.23.366-1.427.366-1.427.18.344.707.646 1.268.646 1.67 0 2.803-1.521 2.803-3.56-.001-1.538-1.306-2.974-3.289-2.974z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialPinterestCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialSkype")]
        TiIcon::TiSocialSkype => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.668 13.312c.059-.427.089-.846.089-1.251 0-2.437-.846-4.544-2.513-6.263-1.685-1.735-3.755-2.615-6.152-2.615-.279 0-.546.013-.801.038-.756-.35-1.568-.526-2.426-.526-1.609 0-3.053.61-4.174 1.765-1.105 1.135-1.691 2.585-1.691 4.192 0 .832.156 1.619.466 2.348-.047.357-.07.713-.07 1.062 0 2.438.853 4.547 2.532 6.267 1.693 1.732 3.768 2.61 6.164 2.61.254 0 .547-.02.896-.06.69.283 1.409.426 2.146.426 1.588 0 3.025-.614 4.157-1.777 1.117-1.143 1.709-2.6 1.709-4.211 0-.677-.111-1.349-.332-2.005zm-5.168 2.036c-.284.427-.729.781-1.339 1.065-.609.243-1.31.365-2.1.365-.954 0-1.756-.173-2.404-.519-.467-.262-.833-.598-1.096-1.003-.284-.447-.428-.862-.428-1.248 0-.243.092-.457.274-.639.184-.184.416-.274.7-.274.203 0 .406.072.609.213.162.162.283.366.364.609.021.02.153.253.396.7.102.141.284.283.547.425.245.122.568.183.975.183.548 0 1.005-.121 1.37-.364.324-.224.486-.507.486-.853 0-.284-.092-.498-.274-.639-.183-.184-.415-.324-.699-.427l-.319-.075-.457-.123c-.172-.051-.32-.086-.441-.106-.689-.141-1.277-.324-1.766-.548-.426-.162-.811-.445-1.156-.851-.283-.386-.426-.843-.426-1.37 0-.528.152-.994.457-1.4.304-.406.74-.711 1.308-.913.569-.224 1.219-.334 1.949-.334.548 0 1.066.07 1.552.213.468.162.843.355 1.127.579.263.243.477.486.639.729.142.324.214.589.214.791 0 .265-.092.488-.275.669-.201.204-.436.306-.699.306-.223 0-.416-.061-.578-.184-.102-.101-.233-.283-.396-.547-.121-.284-.314-.537-.577-.762-.243-.182-.619-.273-1.127-.273-.507 0-.892.102-1.156.305-.284.184-.426.396-.426.639 0 .162.05.296.152.396.102.143.232.255.396.336.102.06.274.132.518.212l.41.106c.213.053.391.087.533.107.283.061.771.192 1.461.396.406.141.791.323 1.156.548.346.242.599.517.762.82.162.365.242.771.242 1.217-.002.548-.154 1.056-.458 1.523z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialSkype".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialSkypeOutline")]
        TiIcon::TiSocialSkypeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8.865 5c.751 0 1.44.202 2.069.609.324-.08.711-.121 1.157-.121 1.846 0 3.418.67 4.717 2.008 1.299 1.339 1.948 2.962 1.948 4.87 0 .466-.051.953-.152 1.461.264.589.396 1.187.396 1.794 0 1.097-.38 2.036-1.142 2.815-.761.781-1.668 1.173-2.725 1.173-.629 0-1.237-.162-1.825-.488-.527.081-.933.122-1.217.122-1.847 0-3.425-.67-4.733-2.009s-1.963-2.962-1.963-4.868c0-.447.051-.902.152-1.37-.364-.609-.547-1.288-.547-2.039 0-1.096.376-2.029 1.126-2.799.75-.772 1.664-1.158 2.739-1.158m3.135 10.53c-.406 0-.729-.061-.975-.183-.263-.142-.445-.284-.547-.425-.243-.447-.376-.681-.396-.7-.081-.243-.202-.447-.364-.609-.203-.14-.406-.213-.61-.213-.284 0-.517.091-.7.274-.183.182-.274.396-.274.639 0 .386.144.801.428 1.248.263.405.629.741 1.096 1.003.648.346 1.45.519 2.404.519.79 0 1.49-.122 2.1-.365.609-.284 1.055-.639 1.339-1.065.304-.467.456-.975.456-1.522 0-.445-.08-.852-.242-1.217-.163-.304-.416-.578-.762-.82-.365-.225-.75-.407-1.156-.548-.689-.204-1.178-.336-1.461-.396-.143-.021-.32-.055-.533-.107l-.41-.106c-.243-.08-.416-.152-.518-.212-.163-.081-.294-.193-.396-.336-.103-.1-.152-.233-.152-.396 0-.243.142-.455.426-.639.265-.203.649-.305 1.156-.305.508 0 .884.092 1.127.273.263.225.456.478.577.762.163.264.295.446.396.547.162.123.355.184.578.184.264 0 .498-.102.699-.306.184-.181.275-.404.275-.669 0-.202-.072-.467-.214-.791-.162-.243-.376-.486-.639-.729-.284-.224-.659-.417-1.127-.579-.485-.143-1.004-.213-1.552-.213-.73 0-1.38.11-1.948.335-.566.2-1.003.505-1.307.911-.305.406-.457.872-.457 1.4 0 .527.143.984.426 1.37.346.405.73.688 1.156.851.488.224 1.076.407 1.766.548.121.021.27.056.441.106l.457.123.319.075c.284.103.517.243.699.427.183.141.274.354.274.639 0 .346-.162.629-.486.853-.364.243-.821.364-1.369.364m-3.135-12.53c-1.609 0-3.053.61-4.174 1.765-1.105 1.134-1.691 2.585-1.691 4.192 0 .832.156 1.619.466 2.348-.047.357-.07.713-.07 1.062 0 2.438.853 4.547 2.532 6.267 1.693 1.732 3.768 2.61 6.164 2.61.254 0 .547-.02.896-.06.69.283 1.409.426 2.146.426 1.588 0 3.025-.614 4.157-1.777 1.117-1.143 1.709-2.6 1.709-4.211 0-.677-.111-1.349-.332-2.004.059-.427.089-.846.089-1.251 0-2.437-.846-4.544-2.513-6.263-1.685-1.735-3.755-2.615-6.152-2.615-.279 0-.546.013-.801.038-.756-.35-1.568-.527-2.426-.527z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialSkypeOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialTumbler")]
        TiIcon::TiSocialTumbler => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15.527 17.921v-2.066c-.669.448-1.32.67-1.952.67-.298 0-.631-.094-1.004-.277-.223-.151-.354-.317-.393-.503-.11-.224-.178-.708-.178-1.454v-3.291h3v-2h-3v-3.356h-1.772c-.149.782-.298 1.338-.448 1.673-.184.41-.482.782-.891 1.116-.411.337-.837.577-1.285.725v1.842h1.396v4.521c0 .52.073.964.223 1.337.111.298.334.595.671.893.259.262.631.484 1.115.67.595.15 1.114.223 1.562.223.52 0 1.004-.056 1.45-.167.521-.112 1.023-.298 1.506-.556\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialTumbler".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialTumblerCircular")]
        TiIcon::TiSocialTumblerCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M14.377 15.59v-1.234c-.399.268-.788.4-1.166.4-.178 0-.377-.057-.6-.166-.134-.09-.211-.189-.234-.301-.066-.133-.1-.422-.1-.867v-1.966h1.834v-1.233h-1.834v-1.967h-1.066c-.089.467-.178.8-.267 1-.11.244-.288.467-.533.666-.245.201-.5.345-.767.434v1.101h.833v2.7c0 .311.044.576.134.799.066.178.199.355.4.533.154.156.377.289.666.4.355.09.666.133.934.133.311 0 .6-.033.866-.1.312-.067.612-.178.9-.332\" />\n  <path d=\"M12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialTumblerCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialTwitter")]
        TiIcon::TiSocialTwitter => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.89 7.012c.808-.496 1.343-1.173 1.605-2.034-.786.417-1.569.703-2.351.861-.703-.756-1.593-1.14-2.66-1.14-1.043 0-1.924.366-2.643 1.078-.715.717-1.076 1.588-1.076 2.605 0 .309.039.585.117.819-3.076-.105-5.622-1.381-7.628-3.837-.34.601-.51 1.213-.51 1.846 0 1.301.549 2.332 1.645 3.089-.625-.053-1.176-.211-1.645-.47 0 .929.273 1.705.82 2.388.549.676 1.254 1.107 2.115 1.291-.312.08-.641.118-.979.118-.312 0-.533-.026-.664-.083.23.757.664 1.371 1.291 1.841.625.472 1.344.721 2.152.743-1.332 1.045-2.855 1.562-4.578 1.562-.422 0-.721-.006-.902-.038 1.697 1.102 3.586 1.649 5.676 1.649 2.139 0 4.029-.542 5.674-1.626 1.645-1.078 2.859-2.408 3.639-3.974.784-1.564 1.172-3.192 1.172-4.892v-.468c.758-.57 1.371-1.212 1.84-1.921-.68.293-1.383.492-2.11.593z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialTwitter".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialTwitterCircular")]
        TiIcon::TiSocialTwitterCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M15.279 10.283c.358-.221.597-.521.713-.904-.349.186-.697.312-1.045.383-.312-.336-.708-.507-1.182-.507-.464 0-.855.163-1.175.479-.317.318-.478.706-.478 1.158 0 .137.017.26.052.364-1.368-.048-2.499-.614-3.391-1.706-.151.267-.227.539-.227.82 0 .578.244 1.036.73 1.373-.277-.023-.521-.094-.73-.209 0 .413.121.758.365 1.062.243.3.557.492.939.573-.139.036-.285.053-.435.053-.14 0-.237-.012-.296-.037.104.337.296.609.574.818.277.21.597.32.957.33-.591.465-1.269.694-2.035.694-.188 0-.32-.002-.4-.017.754.489 1.594.733 2.521.733.951 0 1.792-.241 2.522-.723.73-.479 1.271-1.07 1.617-1.767.348-.695.521-1.419.521-2.174v-.209c.336-.253.609-.538.818-.854-.298.133-.611.222-.935.267zM12 21c-2.49 0-4.635-.89-6.376-2.646-1.741-1.751-2.624-3.889-2.624-6.354 0-2.488.884-4.634 2.627-6.375 1.74-1.741 3.884-2.625 6.373-2.625 2.466 0 4.604.883 6.354 2.624 1.756 1.742 2.646 3.888 2.646 6.376 0 2.465-.889 4.604-2.644 6.357-1.751 1.753-3.889 2.643-6.356 2.643zm0-16c-1.966 0-3.588.667-4.958 2.04-1.375 1.372-2.042 2.994-2.042 4.96 0 1.944.668 3.562 2.043 4.945 1.372 1.383 2.993 2.055 4.957 2.055 1.943 0 3.56-.673 4.942-2.057 1.385-1.384 2.058-3.002 2.058-4.943 0-1.963-.672-3.585-2.055-4.957-1.383-1.375-3-2.043-4.945-2.043z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialTwitterCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialVimeo")]
        TiIcon::TiSocialVimeo => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18.92 8.776c-.329 1.929-1.211 3.758-2.649 5.48-1.436 1.725-2.71 2.957-3.819 3.695-.699.331-1.293.34-1.786.034-.493-.31-.883-.751-1.169-1.325-.165-.328-.565-1.569-1.202-3.728-.636-2.155-1.017-3.315-1.139-3.479-.083-.163-.288-.184-.616-.061-.33.122-.555.226-.678.309-.123.081-.226.165-.308.245l-.554-.737.616-.74c.452-.492 1.026-1.007 1.724-1.54.7-.534 1.314-.862 1.848-.987.371-.08.679-.028.924.156.247.184.452.484.616.894.165.409.289.811.369 1.199.083.392.165.854.248 1.387.081.534.164.945.246 1.232.451 1.93.821 2.896 1.109 2.896.41 0 1.067-.863 1.971-2.59.41-.779.432-1.426.062-1.941-.369-.512-.943-.522-1.724-.029.123-.78.472-1.456 1.046-2.031 1.026-1.109 2.157-1.521 3.388-1.234 1.273.247 1.765 1.213 1.477 2.895z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialVimeo".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialVimeoCircular")]
        TiIcon::TiSocialVimeoCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M14.463 9.141c-.512 0-.988.238-1.43.715-.311.312-.5.678-.566 1.101.207-.131.387-.196.541-.196.159 0 .29.07.393.212.199.278.188.629-.033 1.051-.489.934-.846 1.4-1.066 1.4-.156 0-.356-.522-.602-1.567-.043-.155-.088-.378-.133-.667-.045-.288-.088-.538-.133-.75-.045-.211-.111-.428-.2-.649s-.2-.384-.333-.483c-.094-.069-.202-.104-.327-.104l-.173.021c-.289.067-.623.245-1 .534-.379.288-.689.566-.934.833l-.334.4.301.399.166-.133c.066-.045.189-.101.367-.167l.191-.043c.069 0 .116.025.143.076.066.089.271.717.615 1.884.346 1.167.562 1.839.65 2.017.156.311.367.55.633.717.13.08.271.121.427.121.165 0 .346-.047.54-.139.601-.399 1.289-1.066 2.067-2 .778-.933 1.255-1.922 1.433-2.966.156-.911-.11-1.434-.799-1.567-.138-.035-.272-.05-.404-.05zM12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSocialVimeoCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialYoutube")]
        TiIcon::TiSocialYoutube => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22.8 8.6c-.2-1.5-.4-2.6-1-3-.6-.5-5.8-.6-9.8-.6s-9.2.1-9.8.6c-.6.4-.8 1.5-1 3s-.2 2.4-.2 3.4 0 1.9.2 3.4.4 2.6 1 3c.6.5 5.8.6 9.8.6 4 0 9.2-.1 9.8-.6.6-.4.8-1.5 1-3s.2-2.4.2-3.4 0-1.9-.2-3.4zm-12.8 7v-7.2l6 3.6-6 3.6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSocialYoutube".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSocialYoutubeCircular")]
        TiIcon::TiSocialYoutubeCircular => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<polygon points=\"8.48,13.14 9.21,13.14 9.21,16.75 9.91,16.75 9.91,13.14 10.64,13.14 10.64,12.53 8.48,12.53 \" />\n<path d=\"M12.17,16c-0.12,0.14-0.53,0.42-0.53,0.02v-2.39h-0.62v2.61c0,0.79,0.79,0.58,1.16,0.17v0.34h0.62v-3.12h-0.62V16H12.17z\" />\n<path d=\"M14.48,13.61c-0.36,0-0.59,0.27-0.59,0.27v-1.36h-0.63v4.23h0.63v-0.24c0,0,0.21,0.28,0.59,0.28c0.33,0,0.58-0.29,0.58-0.69\n\tc0,0,0-1.26,0-1.73S14.84,13.61,14.48,13.61z M14.41,16.02c0,0.23-0.16,0.34-0.37,0.25c-0.05-0.02-0.1-0.06-0.15-0.11v-1.94\n\tc0.04-0.04,0.09-0.07,0.13-0.1c0.22-0.11,0.39,0.06,0.39,0.29L14.41,16.02L14.41,16.02z\" />\n<path d=\"M16.72,15.86c0,0.24-0.13,0.4-0.28,0.41c-0.16,0.01-0.32-0.12-0.32-0.41v-0.59h1.19v-0.8c0-0.29-0.11-0.51-0.26-0.66\n\tc-0.17-0.16-0.4-0.24-0.63-0.24c-0.22,0-0.45,0.07-0.63,0.21c-0.19,0.15-0.31,0.38-0.31,0.69v1.4c0,0.28,0.09,0.5,0.23,0.66\n\tc0.17,0.18,0.4,0.28,0.64,0.29c0.29,0.01,0.6-0.11,0.78-0.36c0.11-0.15,0.18-0.35,0.18-0.59v-0.16h-0.59\n\tC16.72,15.71,16.72,15.76,16.72,15.86z M16.12,14.47c0-0.17,0.1-0.37,0.29-0.37s0.31,0.18,0.31,0.37s0,0.32,0,0.32h-0.6\n\tC16.12,14.78,16.12,14.64,16.12,14.47z\" />\n<path d=\"M12.97,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9S17.94,3,12.97,3z M14.55,6.37h0.8v2.68c0,0.17,0.08,0.17,0.11,0.17\n\tc0.12,0,0.3-0.13,0.39-0.22V6.36h0.8V9.9h-0.8V9.59c-0.11,0.1-0.22,0.18-0.33,0.24c-0.15,0.08-0.29,0.11-0.43,0.11\n\tc-0.18,0-0.31-0.06-0.41-0.17c-0.09-0.11-0.13-0.28-0.13-0.49V6.37z M12,7.3c0-0.55,0.45-1,1-1s1,0.45,1,1V9c0,0.55-0.45,1-1,1\n\ts-1-0.45-1-1V7.3z M9.92,5.15l0.48,1.76l0.49-1.76h0.91l-0.94,2.78V9.9H9.97V7.93L9.01,5.15H9.92z M17.82,17.69\n\tc-0.51,0.5-4.83,0.51-4.83,0.51s-4.31-0.01-4.83-0.51c-0.51-0.5-0.51-2.99-0.51-3.01c0-0.01,0-2.5,0.51-3.01\n\tc0.51-0.5,4.83-0.51,4.83-0.51s4.31,0.01,4.83,0.51c0.51,0.5,0.52,2.99,0.52,3.01C18.34,14.68,18.34,17.18,17.82,17.69z\" />\n<path d=\"M12.98,9.35c0.17,0,0.25-0.1,0.26-0.26v-1.9c0-0.13-0.13-0.24-0.25-0.24s-0.25,0.1-0.25,0.24v1.9\n\tC12.74,9.24,12.81,9.34,12.98,9.35z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSocialYoutubeCircular".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSortAlphabetically")]
        TiIcon::TiSortAlphabetically => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.895 16.553l-4-8c-.339-.678-1.45-.678-1.789 0l-4 8c-.247.494-.047 1.095.447 1.342.495.248 1.095.046 1.342-.447l.723-1.448h4.764l.724 1.447c.175.351.528.553.895.553.15 0 .303-.034.446-.105.494-.248.695-.848.448-1.342zm-6.277-2.553l1.382-2.764 1.382 2.764h-2.764zM22 18h-6c-.379 0-.725-.214-.895-.553s-.132-.744.095-1.047l4.8-6.4h-4c-.552 0-1-.448-1-1s.448-1 1-1h6c.379 0 .725.214.895.553s.132.744-.095 1.047l-4.8 6.4h4c.552 0 1 .448 1 1s-.448 1-1 1zM14 14h-2c-.552 0-1-.448-1-1s.448-1 1-1h2c.552 0 1 .448 1 1s-.448 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSortAlphabetically".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSortAlphabeticallyOutline")]
        TiIcon::TiSortAlphabeticallyOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M5.618 14h2.764l-1.382-2.764zM21 14l2.4-3.2c.686-.915.795-2.119.284-3.142-.512-1.023-1.54-1.658-2.684-1.658h-6c-1.654 0-3 1.346-3 3 0 .77.295 1.469.774 2h-.274c-.368 0-.708.107-1.005.281l-1.811-3.623c-.498-.995-1.527-1.614-2.684-1.614s-2.186.619-2.684 1.614l-4 8c-.358.717-.416 1.53-.163 2.291s.788 1.376 1.504 1.735c.414.207.879.316 1.342.316 1.143 0 2.171-.635 2.684-1.657l.171-.343h2.291l.171.342c.512 1.023 1.54 1.658 2.684 1.658.464 0 .928-.109 1.342-.316.243-.122.455-.282.652-.458.54.488 1.246.774 2.006.774h6c1.654 0 3-1.346 3-3s-1.346-3-3-3zm-9.553 3.895c-.143.071-.296.105-.446.105-.368 0-.721-.203-.896-.553l-.723-1.447h-4.764l-.724 1.447c-.175.35-.528.553-.895.553-.15 0-.303-.034-.446-.106-.494-.247-.694-.848-.447-1.342l4-8c.169-.338.532-.508.894-.508s.725.169.895.508l4 8c.247.495.046 1.095-.448 1.343zm1.053-3.895c-.552 0-1-.448-1-1s.448-1 1-1h1c.552 0 1 .448 1 1s-.448 1-1 1h-1zm8.5 4h-6c-.379 0-.725-.214-.895-.553s-.132-.744.095-1.047l4.8-6.4h-4c-.552 0-1-.448-1-1s.448-1 1-1h6c.379 0 .725.214.895.553s.132.744-.095 1.047l-4.8 6.4h4c.552 0 1 .448 1 1s-.448 1-1 1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSortAlphabeticallyOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSortNumerically")]
        TiIcon::TiSortNumerically => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M4 18c-.552 0-1-.448-1-1v-6.382l-.553.276c-.495.248-1.095.046-1.342-.447-.247-.494-.046-1.094.448-1.342l2-1c.31-.155.678-.139.973.044.294.183.474.504.474.851v8c0 .552-.448 1-1 1zM13 18h-5c-.404 0-.769-.244-.924-.617-.155-.374-.069-.804.217-1.09l4-4c.254-.254.394-.591.394-.95 0-.358-.14-.695-.394-.949-.508-.508-1.39-.508-1.9.001-.253.252-.393.589-.393.948 0 .552-.448 1-1 1s-1-.448-1-1c0-.894.348-1.733.98-2.364 1.265-1.263 3.464-1.263 4.727.001.632.631.979 1.471.979 2.363 0 .893-.348 1.733-.979 2.364l-2.293 2.293h2.586c.552 0 1 .448 1 1s-.448 1-1 1zM20.955 12.377c.338-.457.545-1.016.545-1.627 0-1.517-1.234-2.75-2.75-2.75-1.031 0-1.966.569-2.44 1.484-.254.49-.063 1.094.428 1.348.49.254 1.094.062 1.348-.428.128-.249.383-.404.664-.404.414 0 .75.336.75.75s-.336.75-.75.75c-.552 0-1 .448-1 1s.448 1 1 1c.689 0 1.25.561 1.25 1.25s-.561 1.25-1.25 1.25-1.25-.561-1.25-1.25c0-.552-.448-1-1-1s-1 .448-1 1c0 1.792 1.458 3.25 3.25 3.25s3.25-1.458 3.25-3.25c0-.939-.406-1.779-1.045-2.373z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSortNumerically".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSortNumericallyOutline")]
        TiIcon::TiSortNumericallyOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M23.292 12.134c.138-.445.208-.91.208-1.384 0-2.619-2.131-4.75-4.75-4.75-1.396 0-2.685.61-3.573 1.632l-.056-.067c-.973-.974-2.349-1.533-3.776-1.533-1.422 0-2.794.556-3.77 1.525-.264-.431-.644-.813-1.122-1.108-.474-.294-1.032-.449-1.613-.449-.482 0-.955.109-1.369.316l-1.406.747c-1.442.721-2.051 2.526-1.313 4.002.272.543.714.982 1.248 1.272v4.663c0 1.654 1.346 3 3 3 .766 0 1.458-.297 1.989-.771.54.487 1.25.771 2.011.771h5c.778 0 1.479-.305 2.01-.795.796.5 1.731.795 2.74.795 2.895 0 5.25-2.355 5.25-5.25 0-.922-.25-1.825-.708-2.616zm-17.292 4.866c0 .552-.448 1-1 1s-1-.448-1-1v-6.382c-.144.072-.306.106-.471.106-.401 0-.813-.203-.988-.553-.247-.494-.031-1.095.463-1.342l1.361-.724c.141-.07.307-.105.475-.105.199 0 .4.05.561.149.294.183.599.504.599.851v8zm8 1h-5c-.404 0-.769-.244-.924-.617-.155-.374-.069-.804.217-1.09l4-4c.254-.254.394-.591.394-.95 0-.358-.14-.695-.394-.949s-.601-.381-.949-.381-.696.127-.952.382c-.252.252-.392.589-.392.948 0 .552-.448 1-1 1s-1-.448-1-1c0-.894.348-1.733.98-2.364.632-.631 1.498-.947 2.364-.947s1.731.316 2.363.948c.632.631.979 1.471.979 2.363 0 .893-.348 1.733-.979 2.364l-2.293 2.293h2.586c.552 0 1 .448 1 1s-.448 1-1 1zm4.75 0c-1.792 0-3.25-1.458-3.25-3.25 0-.552.448-1 1-1s1 .448 1 1c0 .689.561 1.25 1.25 1.25s1.25-.561 1.25-1.25-.561-1.25-1.25-1.25c-.552 0-1-.448-1-1s.448-1 1-1c.414 0 .75-.336.75-.75s-.336-.75-.75-.75c-.281 0-.536.155-.665.404-.178.343-.527.54-.889.54-.155 0-.312-.036-.459-.112-.491-.254-.682-.857-.428-1.348.475-.915 1.41-1.484 2.441-1.484 1.516 0 2.75 1.233 2.75 2.75 0 .611-.207 1.17-.545 1.627.639.594 1.045 1.434 1.045 2.373 0 1.792-1.458 3.25-3.25 3.25z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title
                                    .unwrap_or_else(|| "TiSortNumericallyOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSpanner")]
        TiIcon::TiSpanner => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.285 7.119c-.05-.168-.184-.299-.354-.344-.172-.047-.352.003-.477.126l-2.616 2.557-1.914-.383-.381-1.907 2.645-2.585c.126-.123.178-.303.137-.474s-.168-.308-.336-.361c-.531-.166-1.018-.248-1.489-.248-2.757 0-5 2.243-5 5 0 .323.038.65.118 1.01-.562.463-1.096.862-1.701 1.314-.865.646-1.845 1.377-3.182 2.506-.785.686-1.235 1.659-1.235 2.67 0 1.93 1.57 3.5 3.5 3.5 1.021 0 1.993-.456 2.662-1.25 1.149-1.347 1.891-2.336 2.544-3.209.442-.591.832-1.111 1.283-1.66.36.081.688.119 1.011.119 2.757 0 5-2.243 5-5 0-.437-.068-.875-.215-1.381zm-12.285 9.881c-.553 0-1-.447-1-1s.447-1 1-1 1 .447 1 1-.447 1-1 1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSpanner".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSpannerOutline")]
        TiIcon::TiSpannerOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <circle cx=\"8\" cy=\"16\" r=\"1\" />\n  <path d=\"M20.733 4.657c-.392-.378-1.013-.377-1.399.009.387-.386.388-1.008.01-1.4-1.078-.792-2.405-1.266-3.844-1.266-3.59 0-6.5 2.91-6.5 6.5l.031.379c-.337.239-2.893 2.147-4.258 3.301-1.135.99-1.773 2.375-1.773 3.82 0 2.757 2.243 5 5 5 1.465 0 2.854-.65 3.811-1.784 1.173-1.375 3.08-3.923 3.317-4.229l.372.013c3.59 0 6.5-2.91 6.5-6.5 0-1.44-.474-2.766-1.267-3.843zm-12.733 14.343c-1.656 0-3-1.343-3-3 0-.92.423-1.732 1.064-2.292 2.368-2.002 3.617-2.748 5.115-4.015-.105-.382-.179-.777-.179-1.193 0-2.485 2.015-4.5 4.5-4.5.47 0 .914.092 1.339.226l-2.839 2.774.5 2.5 2.5.5 2.805-2.741c.115.396.195.807.195 1.241 0 2.485-2.015 4.5-4.5 4.5-.416 0-.811-.074-1.193-.18-1.267 1.498-2.013 2.748-4.024 5.105-.551.652-1.363 1.075-2.283 1.075zm11.384-12.729l-2.705 2.645-1.329-.266-.263-1.314 2.726-2.663c.651.393 1.19.939 1.571 1.598z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiSpannerOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSpiral")]
        TiIcon::TiSpiral => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 11.8c1-.4.7-1.8 0-2.4-1.1-.9-2.7-.4-3.4.8-1.5 2.4.9 5 3.4 4.9 2.7-.2 4.3-2.9 3.7-5.4-.7-3-3.9-4.5-6.7-3.6-2.6.8-4.2 3.5-4 6.2.3 3 2.6 5.4 5.5 5.9 2.8.5 5.7-.8 7.2-3.2.7-1.1 1.2-2.4 1.2-3.8 0-.5.5-1 1.1-.9.8 0 1 .8.9 1.4-.4 4.7-4.5 8.6-9.3 8.6-5.9 0-10.5-6.2-8-11.8 2.5-5.4 10.3-6.5 13.3-1.2 1.5 2.5 1.2 5.8-.9 7.9-2 2-5.3 2.4-7.7.7-2.2-1.6-2.9-4.9-1.1-7.2 1.7-2.3 5.5-2.4 7 .2 1.1 1.9 0 5.2-2.5 4.9-1.6 0-3-1.7-2.1-3.2.6-.9 1.9-.6 2.3.1.2.8.1 1.1.1 1.1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSpiral".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStar")]
        TiIcon::TiStar => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9.362 9.158l-5.268.584c-.19.023-.358.15-.421.343s0 .394.14.521c1.566 1.429 3.919 3.569 3.919 3.569-.002 0-.646 3.113-1.074 5.19-.036.188.032.387.196.506.163.119.373.121.538.028 1.844-1.048 4.606-2.624 4.606-2.624l4.604 2.625c.168.092.378.09.541-.029.164-.119.232-.318.195-.505l-1.071-5.191 3.919-3.566c.14-.131.202-.332.14-.524s-.23-.319-.42-.341c-2.108-.236-5.269-.586-5.269-.586l-2.183-4.83c-.082-.173-.254-.294-.456-.294s-.375.122-.453.294l-2.183 4.83z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiStar".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStarFullOutline")]
        TiIcon::TiStarFullOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M3.1 11.3l3.6 3.3-1 4.6c-.1.6.1 1.2.6 1.5.2.2.5.3.8.3.2 0 .4 0 .6-.1 0 0 .1 0 .1-.1l4.1-2.3 4.1 2.3s.1 0 .1.1c.5.2 1.1.2 1.5-.1.5-.3.7-.9.6-1.5l-1-4.6c.4-.3 1-.9 1.6-1.5l1.9-1.7.1-.1c.4-.4.5-1 .3-1.5s-.6-.9-1.2-1h-.1l-4.7-.5-1.9-4.3s0-.1-.1-.1c-.1-.7-.6-1-1.1-1-.5 0-1 .3-1.3.8 0 0 0 .1-.1.1l-1.9 4.3-4.7.5h-.1c-.5.1-1 .5-1.2 1-.1.6 0 1.2.4 1.6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiStarFullOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStarHalf")]
        TiIcon::TiStarHalf => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.5 4.3c-.9 1.9-2.2 4.8-2.2 4.8s-3.1.4-5.2.6c-.2 0-.4.2-.4.3-.1.2 0 .4.1.5 1.6 1.4 3.9 3.6 3.9 3.6s-.6 3.1-1.1 5.2c0 .2 0 .4.2.5.2.2.4.2.6.1 1.8-1 4.6-2.6 4.6-2.6v-13.3c-.2 0-.4.2-.5.3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiStarHalf".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStarHalfOutline")]
        TiIcon::TiStarHalfOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M3.1 11.3l3.6 3.3-1 4.6c-.1.6.1 1.2.6 1.5.2.2.5.3.8.3.2 0 .4 0 .6-.1 0 0 .1 0 .1-.1l4.1-2.3 4.1 2.3s.1 0 .1.1c.5.2 1.1.2 1.5-.1.5-.3.7-.9.6-1.5l-1-4.6c.4-.3 1-.9 1.6-1.5l1.9-1.7.1-.1c.4-.4.5-1 .3-1.5s-.6-.9-1.2-1h-.1l-4.7-.5-1.9-4.3s0-.1-.1-.1c-.1-.7-.6-1-1.1-1-.5 0-1 .3-1.3.8 0 0 0 .1-.1.1l-1.9 4.3-4.7.5h-.1c-.5.1-1 .5-1.2 1-.1.6 0 1.2.4 1.6zm8.9 5v-10.5l1.7 3.8c.1.3.5.5.8.6l4.2.5-3.1 2.8c-.3.2-.4.6-.3 1 0 .2.5 2.2.8 4.1l-3.6-2.1c-.2-.2-.3-.2-.5-.2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiStarHalfOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStarOutline")]
        TiIcon::TiStarOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.855 20.966c-.224 0-.443-.05-.646-.146l-.104-.051-4.107-2.343-4.107 2.344-.106.053c-.488.228-1.085.174-1.521-.143-.469-.34-.701-.933-.586-1.509l.957-4.642-1.602-1.457-1.895-1.725-.078-.082c-.375-.396-.509-.97-.34-1.492.173-.524.62-.912 1.16-1.009l.102-.018 4.701-.521 1.946-4.31.06-.11c.262-.473.764-.771 1.309-.771.543 0 1.044.298 1.309.77l.06.112 1.948 4.312 4.701.521.104.017c.539.1.986.486 1.158 1.012.17.521.035 1.098-.34 1.494l-.078.078-3.498 3.184.957 4.632c.113.587-.118 1.178-.59 1.519-.252.182-.556.281-.874.281zm-8.149-6.564c-.039.182-.466 2.246-.845 4.082l3.643-2.077c.307-.175.684-.175.99 0l3.643 2.075-.849-4.104c-.071-.346.045-.705.308-.942l3.1-2.822-4.168-.461c-.351-.039-.654-.26-.801-.584l-1.728-3.821-1.726 3.821c-.146.322-.45.543-.801.584l-4.168.461 3.1 2.822c.272.246.384.617.302.966z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiStarOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStarburst")]
        TiIcon::TiStarburst => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.064 10.109l1.179-2.387c.074-.149.068-.327-.015-.471-.083-.145-.234-.238-.401-.249l-2.656-.172-.172-2.656c-.011-.167-.104-.317-.249-.401-.145-.084-.322-.09-.472-.015l-2.385 1.18-1.477-2.215c-.186-.278-.646-.278-.832 0l-1.477 2.215-2.385-1.18c-.151-.075-.327-.069-.472.015-.145.083-.238.234-.249.401l-.171 2.656-2.657.171c-.167.011-.318.104-.401.249-.084.145-.089.322-.015.472l1.179 2.386-2.214 1.477c-.139.093-.223.249-.223.416s.083.323.223.416l2.215 1.477-1.18 2.386c-.074.15-.068.327.015.472.083.144.234.238.401.248l2.656.171.171 2.657c.011.167.104.317.249.401.144.083.32.088.472.015l2.386-1.179 1.477 2.214c.093.139.249.223.416.223s.323-.083.416-.223l1.477-2.214 2.386 1.179c.15.073.327.068.472-.015s.238-.234.249-.401l.171-2.656 2.656-.172c.167-.011.317-.104.401-.249.083-.145.089-.322.015-.472l-1.179-2.385 2.214-1.478c.139-.093.223-.249.223-.416s-.083-.323-.223-.416l-2.214-1.475z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiStarburst".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStarburstOutline")]
        TiIcon::TiStarburstOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.556 11.169l-1.849-1.232.984-1.993c.148-.3.137-.654-.03-.943-.168-.29-.468-.477-.802-.498l-2.218-.143-.144-2.218c-.02-.334-.208-.635-.497-.802-.29-.167-.645-.18-.943-.03l-1.991.985-1.233-1.849c-.371-.557-1.293-.557-1.664 0l-1.234 1.848-1.992-.984c-.299-.15-.654-.137-.943.03-.29.167-.477.468-.498.802l-.143 2.217-2.218.143c-.334.022-.635.209-.802.498s-.179.644-.03.943l.984 1.993-1.849 1.233c-.278.186-.445.498-.445.832s.167.646.445.832l1.85 1.233-.985 1.992c-.148.3-.137.654.03.943s.468.477.802.498l2.218.143.143 2.218c.021.333.208.634.498.801s.642.179.943.031l1.992-.985 1.233 1.849c.186.278.498.445.832.445s.646-.167.832-.445l1.233-1.849 1.991.985c.299.148.653.136.943-.03.29-.167.477-.468.498-.802l.143-2.217 2.219-.144c.334-.021.635-.208.802-.498s.179-.644.03-.943l-.984-1.992 1.849-1.233c.278-.186.445-.498.445-.832 0-.334-.167-.647-.445-.832zm-4.032 2.997l.71 1.435-1.6.104c-.502.033-.901.432-.934.934l-.103 1.598-1.435-.709c-.45-.224-.996-.077-1.275.342l-.887 1.33-.889-1.333c-.191-.287-.508-.445-.833-.445-.149 0-.3.033-.442.104l-1.436.709-.103-1.598c-.032-.501-.432-.901-.934-.934l-1.596-.103.71-1.435c.223-.451.076-.997-.342-1.275l-1.333-.889 1.332-.888c.418-.279.564-.825.342-1.275l-.71-1.436 1.6-.103c.502-.033.901-.432.934-.934l.103-1.598 1.435.709c.448.221.996.076 1.275-.342l.887-1.331.889 1.333c.279.418.826.563 1.275.342l1.436-.71.104 1.599c.033.501.433.9.934.933l1.598.103-.709 1.437c-.223.451-.076.996.342 1.275l1.332.888-1.333.889c-.42.277-.566.823-.344 1.274z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiStarburstOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiStopwatch")]
        TiIcon::TiStopwatch => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M19.414 8.902c.104-.048.206-.108.293-.195l.5-.5c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-.5.5-.115.173c-1.387-1.312-3.188-2.19-5.189-2.41l.011-.056v-1h1c.55 0 1-.45 1-1s-.45-1-1-1h-4c-.55 0-1 .45-1 1s.45 1 1 1h1v1l.012.057c-4.506.492-8.012 4.307-8.012 8.943 0 4.971 4.029 9 9 9s9-4.029 9-9c0-1.894-.588-3.648-1.586-5.098zm-7.414 12.098c-3.859 0-7-3.14-7-7s3.141-7 7-7 7 3.14 7 7-3.141 7-7 7zM13 13v-2c0-.55-.45-1-1-1s-1 .45-1 1v3c0 .55.45 1 1 1h3c.55 0 1-.45 1-1s-.45-1-1-1h-2zM12 8c-3.312 0-6 2.688-6 6s2.688 6 6 6 6-2.688 6-6-2.688-6-6-6zm0 11c-2.757 0-5-2.243-5-5s2.243-5 5-5 5 2.243 5 5-2.243 5-5 5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiStopwatch".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiSupport")]
        TiIcon::TiSupport => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 3.5c-4.688 0-8.5 3.812-8.5 8.5s3.812 8.5 8.5 8.5 8.5-3.812 8.5-8.5-3.812-8.5-8.5-8.5zm6.5 8.5c0 1.064-.264 2.066-.718 2.956l-1.931-1.931c.088-.332.147-.674.147-1.025 0-.355-.062-.693-.147-1.021l1.932-1.932c.455.889.717 1.891.717 2.953zm-13 0c0-1.064.264-2.066.718-2.956l1.933 1.933c-.086.33-.147.668-.147 1.022 0 .353.062.69.147 1.021l-1.934 1.934c-.455-.89-.717-1.892-.717-2.954zm3.068-2.02l-1.775-1.775 1.414-1.414 1.775 1.775c-.584.345-1.068.83-1.414 1.414zm-1.777 5.813l1.773-1.773c.17.289.362.564.605.809s.52.438.807.607l-1.771 1.771-1.414-1.414zm3.795-2.379c-.377-.378-.585-.88-.584-1.414 0-1.104.896-2 1.998-2 1.104 0 2 .896 2 2.001.001.533-.207 1.035-.584 1.412-.755.757-2.073.757-2.83.001zm6.623-5.207l-1.775 1.775c-.345-.586-.828-1.069-1.412-1.416l1.773-1.773 1.414 1.414zm-2.378 6.619c.241-.242.435-.518.604-.803l1.771 1.771-1.414 1.414-1.772-1.772c.291-.17.567-.366.811-.61zm.125-8.608l-1.933 1.932c-.328-.088-.668-.15-1.023-.15s-.693.062-1.021.148l-1.932-1.932c.889-.455 1.891-.717 2.953-.717 1.064.001 2.066.265 2.956.719zm-5.912 11.564l1.933-1.933c.332.088.672.149 1.023.149s.691-.062 1.021-.147l1.932 1.932c-.889.455-1.891.717-2.953.717-1.064 0-2.066-.264-2.956-.718z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiSupport".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTabsOutline")]
        TiIcon::TiTabsOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 4h-10c-1.104 0-2 .896-2 2v2h-1c-1.104 0-2 .896-2 2v9c0 1.104.896 2 2 2h9c1.104 0 2-.896 2-2v-1h2c1.104 0 2-.896 2-2v-10c0-1.104-.896-2-2-2zm-13 15v-9h8.5c.275 0 .5.225.5.5v8.5h-9zm13-3h-3v-5.5c0-.827-.673-1.5-1.5-1.5h-5.5v-3h10v10z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTabsOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTag")]
        TiIcon::TiTag => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9 4c1.279 0 2.559.488 3.535 1.465l3.465 3.535 5 5-7 7-5.498-5.498c-.037.033-3.037-2.967-3.037-2.967-1.953-1.953-1.953-5.119 0-7.07.976-.977 2.256-1.465 3.535-1.465m0-2c-1.87 0-3.628.729-4.949 2.051-1.322 1.32-2.051 3.078-2.051 4.948s.729 3.628 2.051 4.95l3 3c.107.107.227.201.35.279l5.187 5.186c.391.391.9.586 1.413.586s1.022-.195 1.414-.586l7-7c.78-.781.78-2.047 0-2.828l-5-5-3.45-3.521c-1.337-1.336-3.095-2.065-4.965-2.065zM9 7.498c.829 0 1.5.672 1.5 1.502s-.671 1.498-1.5 1.498-1.5-.668-1.5-1.498.671-1.502 1.5-1.502m0-1c-1.379 0-2.5 1.122-2.5 2.502 0 1.377 1.121 2.498 2.5 2.498s2.5-1.121 2.5-2.498c0-1.38-1.121-2.502-2.5-2.502z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTag".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTags")]
        TiIcon::TiTags => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M21.422 9.594l-6.465-6.535c-1.329-1.33-3.087-2.059-4.957-2.059s-3.628.729-4.95 2.051c-1.416 1.414-2.127 3.356-2.027 5.314-.662 1.085-1.023 2.33-1.023 3.634 0 1.87.729 3.628 2.051 4.95l3.053 2.984 3.482 3.48c.391.392.902.587 1.414.587s1.023-.195 1.414-.586l7-7c.778-.778.782-2.038.008-2.82l-.093-.094 1.085-1.086c.778-.778.782-2.038.008-2.82zm-9.422 12.406l-3.498-3.497-3.037-2.968c-1.953-1.953-1.953-5.119 0-7.07.976-.977 2.256-1.465 3.535-1.465s2.559.488 3.535 1.465l6.465 6.535-7 7zm1.957-14.941c-1.329-1.33-3.087-2.059-4.957-2.059-1.276 0-2.497.347-3.565.982.241-.55.579-1.067 1.03-1.518.976-.976 2.256-1.464 3.535-1.464s2.559.488 3.535 1.465l6.465 6.535-1.078 1.078-4.965-5.019zM9 10.499c.83 0 1.5.672 1.5 1.501 0 .83-.67 1.499-1.5 1.499s-1.5-.669-1.5-1.499c0-.829.67-1.501 1.5-1.501m0-1c-1.378 0-2.5 1.122-2.5 2.501 0 1.378 1.122 2.499 2.5 2.499s2.5-1.121 2.5-2.499c0-1.379-1.122-2.501-2.5-2.501z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTags".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThLarge")]
        TiIcon::TiThLarge => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M8 3h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881zM18 3h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881zM8 13h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881zM18 13h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThLarge".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThLargeOutline")]
        TiIcon::TiThLargeOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9 2h-5c-1.103 0-2 .896-2 2v5c0 1.104.897 2 2 2h5c1.103 0 2-.896 2-2v-5c0-1.104-.897-2-2-2zm0 7h-5v-5h5v5zM20 2h-5c-1.104 0-2 .896-2 2v5c0 1.104.896 2 2 2h5c1.104 0 2-.896 2-2v-5c0-1.104-.896-2-2-2zm0 7h-5v-5h5v5zM9 13h-5c-1.103 0-2 .896-2 2v5c0 1.104.897 2 2 2h5c1.103 0 2-.896 2-2v-5c0-1.104-.897-2-2-2zm0 7h-5v-5h5v5zM20 13h-5c-1.104 0-2 .896-2 2v5c0 1.104.896 2 2 2h5c1.104 0 2-.896 2-2v-5c0-1.104-.896-2-2-2zm0 7h-5v-5h5v5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiThLargeOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThList")]
        TiIcon::TiThList => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 17h-7c-1.103 0-2 .897-2 2s.897 2 2 2h7c1.103 0 2-.897 2-2s-.897-2-2-2zM19 10h-7c-1.103 0-2 .897-2 2s.897 2 2 2h7c1.103 0 2-.897 2-2s-.897-2-2-2zM19 3h-7c-1.103 0-2 .897-2 2s.897 2 2 2h7c1.103 0 2-.897 2-2s-.897-2-2-2z\" />\n<circle cx=\"5\" cy=\"19\" r=\"2.5\" />\n<circle cx=\"5\" cy=\"12\" r=\"2.5\" />\n<circle cx=\"5\" cy=\"5\" r=\"2.5\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThList".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThListOutline")]
        TiIcon::TiThListOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 18c.55 0 1 .45 1 1s-.45 1-1 1h-7c-.55 0-1-.45-1-1s.45-1 1-1h7m0-2h-7c-1.654 0-3 1.346-3 3s1.346 3 3 3h7c1.654 0 3-1.346 3-3s-1.346-3-3-3zM19 11c.55 0 1 .45 1 1s-.45 1-1 1h-7c-.55 0-1-.45-1-1s.45-1 1-1h7m0-2h-7c-1.654 0-3 1.346-3 3s1.346 3 3 3h7c1.654 0 3-1.346 3-3s-1.346-3-3-3zM19 4c.55 0 1 .45 1 1s-.45 1-1 1h-7c-.55 0-1-.45-1-1s.45-1 1-1h7m0-2h-7c-1.654 0-3 1.346-3 3s1.346 3 3 3h7c1.654 0 3-1.346 3-3s-1.346-3-3-3zM6 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM6 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM6 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThListOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThMenu")]
        TiIcon::TiThMenu => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 17h-14c-1.103 0-2 .897-2 2s.897 2 2 2h14c1.103 0 2-.897 2-2s-.897-2-2-2zM19 10h-14c-1.103 0-2 .897-2 2s.897 2 2 2h14c1.103 0 2-.897 2-2s-.897-2-2-2zM19 3h-14c-1.103 0-2 .897-2 2s.897 2 2 2h14c1.103 0 2-.897 2-2s-.897-2-2-2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThMenu".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThMenuOutline")]
        TiIcon::TiThMenuOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 18c.55 0 1 .45 1 1s-.45 1-1 1h-14c-.55 0-1-.45-1-1s.45-1 1-1h14m0-2h-14c-1.654 0-3 1.346-3 3s1.346 3 3 3h14c1.654 0 3-1.346 3-3s-1.346-3-3-3zM19 11c.55 0 1 .45 1 1s-.45 1-1 1h-14c-.55 0-1-.45-1-1s.45-1 1-1h14m0-2h-14c-1.654 0-3 1.346-3 3s1.346 3 3 3h14c1.654 0 3-1.346 3-3s-1.346-3-3-3zM19 4c.55 0 1 .45 1 1s-.45 1-1 1h-14c-.55 0-1-.45-1-1s.45-1 1-1h14m0-2h-14c-1.654 0-3 1.346-3 3s1.346 3 3 3h14c1.654 0 3-1.346 3-3s-1.346-3-3-3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThMenuOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThSmall")]
        TiIcon::TiThSmall => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<circle cx=\"5\" cy=\"19\" r=\"2.5\" />\n<circle cx=\"5\" cy=\"12\" r=\"2.5\" />\n<circle cx=\"5\" cy=\"5\" r=\"2.5\" />\n<circle cx=\"12\" cy=\"19\" r=\"2.5\" />\n<circle cx=\"12\" cy=\"12\" r=\"2.5\" />\n<circle cx=\"12\" cy=\"5\" r=\"2.5\" />\n<circle cx=\"19\" cy=\"19\" r=\"2.5\" />\n<circle cx=\"19\" cy=\"12\" r=\"2.5\" />\n<circle cx=\"19\" cy=\"5\" r=\"2.5\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThSmall".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThSmallOutline")]
        TiIcon::TiThSmallOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M6 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM6 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM6 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM13 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM13 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM13 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM20 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM20 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM20 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiThSmallOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThermometer")]
        TiIcon::TiThermometer => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M13 15.071v-5.571c0-.275-.225-.5-.5-.5s-.5.225-.5.5v5.571c-.86.224-1.5 1-1.5 1.929 0 1.103.896 2 2 2s2-.897 2-2c0-.929-.64-1.705-1.5-1.929zM16 13.459v-7.959c0-1.93-1.57-3.5-3.5-3.5s-3.5 1.57-3.5 3.5v7.959c-.922.902-1.5 2.151-1.5 3.541 0 2.757 2.243 5 5 5s5-2.243 5-5c0-1.39-.578-2.639-1.5-3.541zm-3.5 6.541c-1.654 0-3-1.346-3-3 0-1.105.607-2.062 1.5-2.583v-8.917c0-.827.673-1.5 1.5-1.5s1.5.673 1.5 1.5v8.917c.893.521 1.5 1.478 1.5 2.583 0 1.654-1.346 3-3 3z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThermometer".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThumbsDown")]
        TiIcon::TiThumbsDown => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 5c-.755 0-1.438.289-1.965.751l-.188-.192c-.96-.737-3.665-1.559-5.847-1.559-1.879 0-2.607.293-3.252.552l-.316.124c-.834.305-1.578 1.229-1.738 2.2l-.664 5.972c-.174 1.039.441 2.127 1.4 2.478.394.144 2.512.405 3.883.56-.215 1.256-.312 2.405-.312 3.616 0 1.379 1.121 2.5 2.5 2.5s2.5-1.121 2.5-2.5c0-1.875.667-2.737 1.616-3.699.548.724 1.408 1.199 2.384 1.199 1.653 0 2.999-1.347 2.999-3v-6c-.001-1.656-1.346-3.002-3-3.002zm-6 14.5c0 .275-.225.5-.5.5s-.5-.225-.5-.5c0-1.805.256-3.241.479-4.293l.297-1.398-1.321.188c-.605-.05-3.934-.447-4.335-.552-.058-.028-.132-.18-.108-.321l.663-5.976c.037-.223.291-.539.443-.594l.377-.146c.544-.219 1.015-.408 2.506-.408 1.914 0 4.118.753 4.633 1.146.156.12.366.564.366.854v4.977c-.001.026-.04.649-.707 1.316-.913.913-2.293 2.293-2.293 5.207zm7-5.5c0 .552-.448 1-1 1s-1-.448-1-1v-6c0-.552.448-1 1-1s1 .448 1 1v6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThumbsDown".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThumbsOk")]
        TiIcon::TiThumbsOk => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.7 12.6c-.1-.3-.2-.6-.4-.9.3-.6.5-1 .3-1.9-.2-.8-.8-1.7-1.6-2.5l-1.7-.9.8-1.8c.4-.8.4-1.8-.1-2.5-.5-.8-1.4-1.3-2.3-1.3-1 0-1.8.6-2.3 1.5 0 0-.8-.2-1.8.1l-1.3 1.2s-1.7.5-2 2-.4 3.4-.8 4.1-2.5 3.3-3.7 4.7c-.9 1.1-.7 2.3.1 3.2l5 5c.8.8 2.7 1.1 4-.3 1.4-1.4.5-3.3.5-3.3.4-.3 1.5-1.2 2.6-1.5s2.8-.9 3.7-2.1c.8-1 1.2-1.8 1-2.8zm-9.3 8.3c-.4.4-1 .4-1.4 0l-4.2-4.2c-.2-.2-.3-.4-.3-.7s.1-.5.3-.7l.7-.7 4.9 4.9c.2.2.3.4.3.7 0 .3-.1.5-.3.7zm6.6-9.9c-.5 0-.7-.5-1.1-.9s-1.3-.5-2-.2-1 1-1 1.7c0 1 .7 1.9 1.7 1.9.7 0 .9-.1 1.3-.3.6-.4.8-.8 1.2-.8s.7.2.7.8-.6 1.3-1.2 1.7-1.1.5-1.7.6c-.6.1-1 .1-1.8.6-.7.4-1.4.9-1.8 1.3l-4.6-4.6c.9-1.1 1.7-2.3 1.8-2.9l.7-3.9c.1-.4.4-.5.6-.5.3 0 .6.2.7.4l.4-1.3c.1-.3.4-.5.6-.5.4 0 .8.3.7.8l-.5 2.4c.6-1.2 1.5-2.7 2.1-3.8.2-.3.4-.6.9-.6s.9.6.7 1.1c-.2.4-2 3.5-2.8 4.8-.1.1 0 .2.2.1.3-.2 1-.7 1.7-.7 1.2-.1 1.8.4 2.1.6.4.3.8.8 1.1 1.3.2.5-.2.9-.7.9zm-.2.7c-.4 0-.7.1-.9.4s-.6.7-1.1.7c-.7 0-1.1-.5-1.1-1.1s.4-1 1.1-1c.5 0 .9.4 1.1.7s.5.3.9.3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThumbsOk".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiThumbsUp")]
        TiIcon::TiThumbsUp => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.57 8.676c-.391-.144-2.512-.406-3.883-.56.215-1.255.313-2.405.313-3.616 0-1.379-1.122-2.5-2.5-2.5s-2.5 1.121-2.5 2.5c0 1.875-.666 2.738-1.616 3.699-.548-.722-1.407-1.199-2.384-1.199-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3 .755 0 1.438-.29 1.965-.752l.188.193c.96.736 3.667 1.559 5.848 1.559 1.879 0 2.608-.293 3.253-.553l.316-.123c.834-.305 1.576-1.227 1.736-2.2l.666-5.974c.173-1.037-.443-2.125-1.402-2.474zm-12.57 8.324c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1s1 .448 1 1v6c0 .552-.449 1-1 1zm11.327-.15c-.037.224-.292.541-.443.596l-.376.146c-.545.219-1.016.408-2.508.408-1.914 0-4.118-.753-4.632-1.146-.158-.12-.368-.564-.368-.854v-4.98c.003-.047.051-.656.707-1.312.913-.914 2.293-2.294 2.293-5.208 0-.275.225-.5.5-.5s.5.225.5.5c0 1.407-.146 2.73-.479 4.293l-.297 1.396 1.321-.188c.603.05 3.933.447 4.334.55.058.03.132.183.111.323l-.663 5.976z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiThumbsUp".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTick")]
        TiIcon::TiTick => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16.972 6.251c-.967-.538-2.185-.188-2.72.777l-3.713 6.682-2.125-2.125c-.781-.781-2.047-.781-2.828 0-.781.781-.781 2.047 0 2.828l4 4c.378.379.888.587 1.414.587l.277-.02c.621-.087 1.166-.46 1.471-1.009l5-9c.537-.966.189-2.183-.776-2.72z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTick".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTickOutline")]
        TiIcon::TiTickOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11 20c-.801 0-1.555-.312-2.121-.879l-4-4c-.567-.566-.879-1.32-.879-2.121s.312-1.555.879-2.122c1.133-1.133 3.109-1.133 4.242 0l1.188 1.188 3.069-5.523c.526-.952 1.533-1.544 2.624-1.544.507 0 1.012.131 1.456.378.7.39 1.206 1.028 1.427 1.798.221.771.127 1.581-.263 2.282l-5 9c-.454.818-1.279 1.384-2.206 1.514-.139.019-.277.029-.416.029zm-4-8c-.268 0-.518.104-.707.293s-.293.439-.293.707.104.518.293.707l4 4c.223.221.523.33.844.283.312-.043.586-.232.737-.504l5-9c.13-.233.161-.503.088-.76-.073-.257-.243-.47-.478-.6-.473-.264-1.101-.078-1.357.388l-4.357 7.841-3.062-3.062c-.19-.189-.44-.293-.708-.293z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTickOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTicket")]
        TiIcon::TiTicket => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M21.485 8.071l-5.364-5.364c-1.128-1.128-3.111-1.136-4.248-.018l-9.148 9.002c-.571.562-.887 1.313-.891 2.115-.003.803.307 1.556.873 2.121l5.365 5.365c.567.567 1.325.88 2.133.88.799 0 1.551-.307 2.115-.862l9.147-9.003c.571-.562.888-1.313.891-2.115.003-.802-.307-1.555-.873-2.121zm-1.421 2.811l-9.146 9.003c-.381.373-1.056.37-1.432-.006l-1.275-1.275c.71-.785.693-1.994-.062-2.752-.758-.757-1.968-.773-2.752-.063l-1.275-1.274c-.186-.187-.288-.435-.287-.699s.105-.513.293-.697l9.148-9.002c.189-.186.441-.288.713-.288.273 0 .529.104.719.294l1.275 1.275c-.711.785-.694 1.994.062 2.751.758.757 1.967.773 2.752.063l1.274 1.274c.187.187.288.435.287.699s-.105.512-.294.697zM11.601 17.042l-4.657-4.656 5.649-5.429 4.657 4.656-5.649 5.429zm-3.23-4.643l3.243 3.242 4.206-4.041-3.241-3.242-4.208 4.041z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTicket".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTime")]
        TiIcon::TiTime => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16 13c0-.55-.45-1-1-1h-3c-.55 0-1 .45-1 1s.45 1 1 1h3c.55 0 1-.45 1-1zM12 6c3.859 0 7 3.141 7 7s-3.141 7-7 7-7-3.141-7-7 3.141-7 7-7m0-2c-4.971 0-9 4.029-9 9s4.029 9 9 9 9-4.029 9-9-4.029-9-9-9zM13 10c0-.55-.45-1-1-1s-1 .45-1 1v3c0 .55.45 1 1 1s1-.45 1-1v-3zM12 8c2.757 0 5 2.243 5 5s-2.243 5-5 5-5-2.243-5-5 2.243-5 5-5m0-1c-3.312 0-6 2.686-6 6 0 3.312 2.688 6 6 6s6-2.688 6-6c0-3.314-2.688-6-6-6z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTime".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTimes")]
        TiIcon::TiTimes => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.414 6.586c-.78-.781-2.048-.781-2.828 0l-2.586 2.586-2.586-2.586c-.78-.781-2.048-.781-2.828 0-.781.781-.781 2.047 0 2.828l2.585 2.586-2.585 2.586c-.781.781-.781 2.047 0 2.828.39.391.902.586 1.414.586s1.024-.195 1.414-.586l2.586-2.586 2.586 2.586c.39.391.902.586 1.414.586s1.024-.195 1.414-.586c.781-.781.781-2.047 0-2.828l-2.585-2.586 2.585-2.586c.781-.781.781-2.047 0-2.828z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTimes".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTimesOutline")]
        TiIcon::TiTimesOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M16 19c-.802 0-1.555-.312-2.122-.879l-1.878-1.879-1.879 1.879c-1.133 1.133-3.109 1.133-4.243 0-.566-.566-.878-1.32-.878-2.121s.312-1.555.879-2.122l1.878-1.878-1.878-1.879c-.567-.566-.879-1.32-.879-2.121s.312-1.555.879-2.122c1.133-1.132 3.109-1.133 4.243.001l1.878 1.879 1.879-1.879c1.133-1.133 3.109-1.133 4.243 0 .566.566.878 1.32.878 2.121s-.312 1.555-.879 2.122l-1.878 1.878 1.878 1.879c.567.566.879 1.32.879 2.121s-.312 1.555-.879 2.122c-.566.566-1.319.878-2.121.878zm-4-5.586l3.293 3.293c.378.378 1.037.377 1.414 0 .189-.189.293-.439.293-.707s-.104-.518-.293-.707l-3.292-3.293 3.292-3.293c.189-.189.293-.44.293-.707s-.104-.518-.293-.707c-.378-.379-1.037-.378-1.414-.001l-3.293 3.294-3.293-3.293c-.378-.378-1.037-.377-1.414 0-.189.189-.293.44-.293.707s.104.518.293.707l3.292 3.293-3.292 3.293c-.189.189-.293.439-.293.707s.104.518.293.707c.378.379 1.037.378 1.414.001l3.293-3.294z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTimesOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTrash")]
        TiIcon::TiTrash => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M18 7h-1v-1c0-1.104-.896-2-2-2h-7c-1.104 0-2 .896-2 2v1h-1c-.552 0-1 .448-1 1s.448 1 1 1v8c0 2.206 1.794 4 4 4h5c2.206 0 4-1.794 4-4v-8c.552 0 1-.448 1-1s-.448-1-1-1zm-10-1h7v1h-7v-1zm8 11c0 1.104-.896 2-2 2h-5c-1.104 0-2-.896-2-2v-8h9v8zM8.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5zM10.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5zM12.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5zM14.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTrash".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiTree")]
        TiIcon::TiTree => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.781 17.375l-2.7-3.375h.919c.373 0 .715-.207.887-.538.172-.331.146-.729-.068-1.035l-7-10c-.317-.452-.94-.562-1.393-.246-.091.063-.158.146-.221.231-.025.015-7.025 10.015-7.025 10.015-.214.306-.24.704-.068 1.035.173.331.515.538.888.538h.919l-2.7 3.375c-.24.301-.287.712-.121 1.059.167.345.518.566.902.566h7v3c0 .553.448 1 1 1s1-.447 1-1v-3h7c.384 0 .735-.221.901-.566.167-.347.12-.758-.12-1.059zm-7.781-.375v-5c0-.553-.448-1-1-1s-1 .447-1 1v5h-4.919l2.7-3.375c.24-.301.287-.712.121-1.059-.167-.345-.518-.566-.902-.566h-1.08l5.08-7.256 5.08 7.256h-1.08c-.384 0-.735.221-.901.566-.167.347-.12.758.121 1.059l2.7 3.375h-4.92z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiTree".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUpload")]
        TiIcon::TiUpload => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.987 16c0-.105-.004-.211-.039-.316l-2-6c-.136-.409-.517-.684-.948-.684h-4v2h3.279l1.667 5h-13.892l1.667-5h3.279v-2h-4c-.431 0-.812.275-.948.684l-2 6c-.035.105-.039.211-.039.316-.013 0-.013 5-.013 5 0 .553.447 1 1 1h16c.553 0 1-.447 1-1 0 0 0-5-.013-5zM16 7.904c.259 0 .518-.095.707-.283.39-.39.39-1.024 0-1.414l-4.707-4.707-4.707 4.707c-.39.39-.39 1.024 0 1.414.189.189.448.283.707.283s.518-.094.707-.283l2.293-2.293v6.672c0 .552.448 1 1 1s1-.448 1-1v-6.672l2.293 2.293c.189.189.448.283.707.283z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiUpload".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUploadOutline")]
        TiIcon::TiUploadOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.986 17c0-.105-.004-.211-.038-.316l-2-6c-.093-.276-.302-.483-.56-.594.881-1.175.799-2.847-.269-3.914l-6.119-6.121-6.121 6.121c-1.067 1.067-1.149 2.739-.27 3.914-.256.109-.467.316-.559.594l-2 6c-.034.105-.038.211-.038.316-.012 0-.012 5-.012 5 0 .553.447 1 1 1h16c.553 0 1-.447 1-1 0 0 0-5-.014-5zm-13.693-9.41l4.707-4.707 4.707 4.707c.391.391.391 1.023 0 1.414-.379.377-1.035.377-1.414 0l-2.293-2.293v5.789c0 .552-.448 1-1 1s-1-.448-1-1v-5.789l-2.293 2.293c-.379.377-1.035.377-1.414 0-.391-.391-.391-1.025 0-1.414zm-.572 4.41h2.279v.5c0 1.654 1.346 3 3 3s3-1.346 3-3v-.5h2.279l1.666 5h-13.892l1.668-5zm-1.721 9v-3h14v3h-14z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiUploadOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUser")]
        TiIcon::TiUser => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 9c0-1.381-.56-2.631-1.464-3.535-.905-.905-2.155-1.465-3.536-1.465s-2.631.56-3.536 1.465c-.904.904-1.464 2.154-1.464 3.535s.56 2.631 1.464 3.535c.905.905 2.155 1.465 3.536 1.465s2.631-.56 3.536-1.465c.904-.904 1.464-2.154 1.464-3.535zM6 19c0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4-3.75 0-6 2-6 4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiUser".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUserAdd")]
        TiIcon::TiUserAdd => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M9 14c1.381 0 2.631-.56 3.536-1.465.904-.904 1.464-2.154 1.464-3.535s-.56-2.631-1.464-3.535c-.905-.905-2.155-1.465-3.536-1.465s-2.631.56-3.536 1.465c-.904.904-1.464 2.154-1.464 3.535s.56 2.631 1.464 3.535c.905.905 2.155 1.465 3.536 1.465zM9 21c3.518 0 6-1 6-2 0-2-2.354-4-6-4-3.75 0-6 2-6 4 0 1 2.25 2 6 2zM21 12h-2v-2c0-.553-.447-1-1-1s-1 .447-1 1v2h-2c-.553 0-1 .447-1 1s.447 1 1 1h2v2c0 .553.447 1 1 1s1-.447 1-1v-2h2c.553 0 1-.447 1-1s-.447-1-1-1z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiUserAdd".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUserAddOutline")]
        TiIcon::TiUserAddOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 14h-6c-.553 0-1-.448-1-1s.447-1 1-1h6c.553 0 1 .448 1 1s-.447 1-1 1zM18 17c-.553 0-1-.448-1-1v-6c0-.552.447-1 1-1s1 .448 1 1v6c0 .552-.447 1-1 1zM9 6c1.654 0 3 1.346 3 3s-1.346 3-3 3-3-1.346-3-3 1.346-3 3-3m0-2c-2.764 0-5 2.238-5 5s2.236 5 5 5 5-2.238 5-5-2.236-5-5-5zM9 17c2.021 0 3.301.771 3.783 1.445-.683.26-1.969.555-3.783.555-1.984 0-3.206-.305-3.818-.542.459-.715 1.777-1.458 3.818-1.458m0-2c-3.75 0-6 2-6 4 0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiUserAddOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUserDelete")]
        TiIcon::TiUserDelete => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 14h-6c-.553 0-1-.447-1-1s.447-1 1-1h6c.553 0 1 .447 1 1s-.447 1-1 1zM14 9c0 1.381-.56 2.631-1.464 3.535-.905.905-2.155 1.465-3.536 1.465s-2.631-.56-3.536-1.465c-.904-.904-1.464-2.154-1.464-3.535s.56-2.631 1.464-3.535c.905-.905 2.155-1.465 3.536-1.465s2.631.56 3.536 1.465c.904.904 1.464 2.154 1.464 3.535zM9 15c-3.75 0-6 2-6 4 0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiUserDelete".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUserDeleteOutline")]
        TiIcon::TiUserDeleteOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21 14h-6c-.553 0-1-.448-1-1s.447-1 1-1h6c.553 0 1 .448 1 1s-.447 1-1 1zM9 6c1.654 0 3 1.346 3 3s-1.346 3-3 3-3-1.346-3-3 1.346-3 3-3m0-2c-2.764 0-5 2.238-5 5s2.236 5 5 5 5-2.238 5-5-2.236-5-5-5zM9 17c2.021 0 3.301.771 3.783 1.445-.683.26-1.969.555-3.783.555-1.984 0-3.206-.305-3.818-.542.459-.715 1.777-1.458 3.818-1.458m0-2c-3.75 0-6 2-6 4 0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiUserDeleteOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiUserOutline")]
        TiIcon::TiUserOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 6c1.654 0 3 1.346 3 3s-1.346 3-3 3-3-1.346-3-3 1.346-3 3-3m0-2c-2.764 0-5 2.238-5 5s2.236 5 5 5 5-2.238 5-5-2.236-5-5-5zM12 17c2.021 0 3.301.771 3.783 1.445-.683.26-1.969.555-3.783.555-1.984 0-3.206-.305-3.818-.542.459-.715 1.777-1.458 3.818-1.458m0-2c-3.75 0-6 2-6 4 0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiUserOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVendorAndroid")]
        TiIcon::TiVendorAndroid => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M7.1 8h9.799999999999999c.1 0 .1 0 .1-.1-.1-.5-.2-.9-.4-1.3-.4-.9-1.1-1.5-1.9-2l-.4-.2s0-.1.1-.1c.3-.4.6-.8.9-1.3.1-.1.1-.2 0-.3-.1-.1-.2 0-.3.1l-.9 1.3s-.1.1-.1 0c-.8-.3-1.6-.4-2.4-.4-.6 0-1.1.2-1.6.4h-.1l-.9-1.2s0-.1-.1-.1c-.1-.1-.2-.1-.2 0-.1 0-.1.1 0 .2 0 0 0 .1.1.1.2.4.5.8.8 1.2l.1.1h-.1c-.6.3-1.1.7-1.5 1.2-.6.6-1 1.4-1.1 2.3 0 .1 0 .1.1.1zm7.1-2.8c.4 0 .8.3.8.8 0 .4-.3.8-.7.8-.4 0-.8-.3-.8-.8-.1-.4.3-.8.7-.8zm-4.3 0c.4 0 .8.3.8.8 0 .4-.3.8-.7.8-.5 0-.9-.4-.9-.8s.4-.8.8-.8zM5 9c-.5 0-1 .5-1 1v5c0 .5.5 1 1 1s1-.5 1-1v-5c0-.5-.5-1-1-1zM19 9c-.5 0-1 .5-1 1v5c0 .5.5 1 1 1s1-.5 1-1v-5c0-.5-.5-1-1-1zM7 17c0 .5.5 1 1 1h1v3c0 .5.5 1 1 1s1-.5 1-1v-3h2v3c0 .5.5 1 1 1s1-.5 1-1v-3h1c.5 0 1-.5 1-1v-8h-10v8z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVendorAndroid".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVendorApple")]
        TiIcon::TiVendorApple => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M11.9 6.6s-.1-1.6.9-3l2.8-1.6s.1 1.6-.9 3l-2.8 1.6zM17.3 12.2c0-1.5.8-2.8 2-3.6l-.9-.9c-.5-.3-1.1-.7-2.4-.7-1.4 0-2.4.9-3.7.9-1.3 0-2.2-.8-3.1-.9-.7 0-1.4 0-2.1.3-.5.2-1.2.7-1.6 1.2-.6.6-1.2 1.9-1.3 3.1-.1 1.2-.1 2.1.2 3.2.2.9.6 1.8 1 2.6.3.6.6 1.2 1 1.8.3.4.7.8 1.1 1.1.3.2.6.4 1 .6.2 0 .5.1.8.1.6-.1 1.6-.9 2.4-1.1.4-.1.8-.1 1.3 0 .7.1 1.4.9 2.2 1 .6.1 1.2 0 1.7-.3.4-.2.7-.5 1-.9.4-.4.7-.9 1-1.3.4-.7.9-1.5 1.1-2.3-1.6-.6-2.7-2.1-2.7-3.9z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVendorApple".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVendorMicrosoft")]
        TiIcon::TiVendorMicrosoft => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10 12.5c0-.3-.2-.5-.5-.5h-6c-.3 0-.5.2-.5.5v5c0 .3.2.5.5.6l6 .7c.3 0 .5-.2.5-.4v-5.9zM11.5 12c-.3 0-.5.2-.5.5v5.9c0 .3.2.5.5.6l9 1c.3 0 .5-.2.5-.4v-7c0-.3-.2-.5-.5-.5l-9-.1zM10 4.7c0-.3-.2-.5-.5-.4l-6 .7c-.3 0-.5.2-.5.5v5c0 .3.2.5.5.5h6c.3 0 .5-.2.5-.5v-5.8zM11.5 4.1c-.3 0-.5.3-.5.6v5.9c0 .3.2.5.5.5h9c.3 0 .5-.2.5-.5v-7c0-.3-.2-.5-.5-.4l-9 .9z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiVendorMicrosoft".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVideo")]
        TiIcon::TiVideo => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M22.525 7.149c-.16-.099-.342-.149-.525-.149-.153 0-.306.035-.447.105l-2.553 1.277v-.382c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v8c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-.382l2.553 1.276c.141.071.294.106.447.106.183 0 .365-.05.525-.149.295-.183.475-.504.475-.851v-8c0-.347-.18-.668-.475-.851zm-15.525 6.351c-.829 0-1.5-.671-1.5-1.5s.671-1.5 1.5-1.5 1.5.671 1.5 1.5-.671 1.5-1.5 1.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVideo".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVideoOutline")]
        TiIcon::TiVideoOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<circle cx=\"7\" cy=\"11\" r=\"1\" />\n<path d=\"M21.585 7.188c-.262-.188-.599-.241-.901-.137l-1.707.568c-.188-1.477-1.451-2.62-2.977-2.62h-11c-1.654 0-3 1.347-3 3v6c0 1.653 1.346 3 3 3h3v1.111l.008.09c.066.738.381 1.423.887 1.928.562.562 1.311.872 2.104.872s1.542-.31 2.104-.87c.574-.577.898-1.346.896-2.113v-1.017h2c1.524 0 2.789-1.145 2.978-2.62l1.707.568c.303.104.64.051.9-.138.262-.188.415-.49.415-.812v-6c.001-.318-.153-.621-.414-.81zm-9.585 10.835c.001.248-.119.5-.309.689-.191.189-.441.286-.692.286-.25 0-.501-.097-.69-.286-.19-.189-.285-.441-.309-.691v-2.021h2v2.023zm5-4.023c0 .552-.448 1-1 1h-11c-.552 0-1-.448-1-1v-6c0-.552.448-1 1-1h11c.552 0 1 .448 1 1v6zm3-1.389s-1.895-.605-2-.605v-2.012c.105 0 2-.605 2-.605v3.222z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVideoOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVolume")]
        TiIcon::TiVolume => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.138 5.824c-.449 0-.905.152-1.356.453l-2.672 1.781c-.753.503-2.206.942-3.11.942-1.654 0-3 1.346-3 3v2c0 1.654 1.346 3 3 3 .904 0 2.357.439 3.109.941l2.672 1.781c.451.301.907.453 1.356.453.898.001 1.863-.68 1.863-2.175v-10c0-1.495-.965-2.176-1.862-2.176zm-3.138 10.322c-1.093-.651-2.789-1.146-4-1.146-.552 0-1-.448-1-1v-2c0-.552.448-1 1-1 1.211 0 2.907-.495 4-1.146v6.292zm3 1.854l-.006.12-.104-.062-1.89-1.26v-7.596l1.891-1.261.104-.062.005.121v10z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVolume".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVolumeDown")]
        TiIcon::TiVolumeDown => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M15.138 5.824c-.449 0-.905.152-1.356.453l-2.672 1.781c-.753.503-2.206.942-3.11.942-1.654 0-3 1.346-3 3v2c0 1.654 1.346 3 3 3 .904 0 2.357.439 3.109.941l2.672 1.781c.451.301.907.453 1.356.453.898.001 1.863-.68 1.863-2.175v-10c0-1.495-.965-2.176-1.862-2.176zm-7.138 9.176c-.552 0-1-.448-1-1v-2c0-.552.448-1 1-1 1.211 0 2.907-.495 4-1.146v6.293c-1.093-.652-2.789-1.147-4-1.147zm7 3l-.006.12-.104-.062-1.89-1.26v-7.596l1.891-1.261.104-.062.005.121v10zM18.292 10.294c-.39.391-.39 1.023.002 1.414.345.345.535.803.535 1.291 0 .489-.19.948-.536 1.294-.391.39-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293c.724-.723 1.122-1.685 1.122-2.708s-.398-1.984-1.123-2.707c-.389-.389-1.023-.391-1.414.002z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVolumeDown".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVolumeMute")]
        TiIcon::TiVolumeMute => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19.707 5.293c-.391-.391-1.023-.391-1.414 0l-1.551 1.551c-.345-.688-.987-1.02-1.604-1.02-.449 0-.905.152-1.356.453l-2.672 1.781c-.753.503-2.206.942-3.11.942-1.654 0-3 1.346-3 3v2c0 1.237.754 2.302 1.826 2.76l-1.533 1.533c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l2.527-2.527c.697.174 1.416.455 1.875.762l2.672 1.781c.451.301.907.453 1.356.453.898 0 1.863-.681 1.863-2.176v-8.586l2.707-2.707c.391-.391.391-1.023 0-1.414zm-4.816 2.648l.104-.062.005.121v1.293l-2 2v-2.091l1.891-1.261zm-7.891 4.059c0-.552.448-1 1-1 1.211 0 2.907-.495 4-1.146v2.439l-2.83 2.83c-.413-.077-.814-.123-1.17-.123-.552 0-1-.448-1-1v-2zm3.301 3.406l1.699-1.699v2.439c-.481-.287-1.075-.542-1.699-.74zm4.693 2.714l-.104-.062-1.89-1.26v-4.091l2-2v7.293l-.006.12z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVolumeMute".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiVolumeUp")]
        TiIcon::TiVolumeUp => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M16.706 10.292c-.389-.389-1.023-.391-1.414.002-.39.391-.39 1.023.002 1.414.345.345.535.803.535 1.291 0 .489-.19.948-.536 1.294-.391.39-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293c.724-.723 1.122-1.685 1.122-2.708s-.398-1.984-1.123-2.707zM18.706 8.292c-.391-.389-1.023-.39-1.414.002-.39.391-.39 1.024.002 1.414.879.877 1.363 2.044 1.364 3.287.001 1.246-.484 2.417-1.365 3.298-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293c1.259-1.259 1.952-2.933 1.951-4.713-.001-1.777-.694-3.447-1.952-4.702zM20.706 6.292c-.391-.389-1.023-.39-1.414.002-.39.391-.39 1.024.002 1.414 1.412 1.409 2.191 3.285 2.192 5.284.002 2.002-.777 3.885-2.193 5.301-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293c1.794-1.794 2.781-4.18 2.779-6.717-.001-2.533-.989-4.912-2.78-6.698zM12.138 5.824c-.449 0-.905.152-1.356.453l-2.673 1.782c-.752.502-2.205.941-3.109.941-1.654 0-3 1.346-3 3v2c0 1.654 1.346 3 3 3 .904 0 2.357.439 3.109.941l2.672 1.781c.451.301.907.453 1.356.453.898.001 1.863-.68 1.863-2.175v-10c0-1.495-.965-2.176-1.862-2.176zm-7.138 9.176c-.552 0-1-.448-1-1v-2c0-.552.448-1 1-1 1.211 0 2.907-.495 4-1.146v6.293c-1.093-.652-2.789-1.147-4-1.147zm7 3l-.006.12-.104-.062-1.89-1.26v-7.596l1.891-1.261.104-.062.005.121v10z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiVolumeUp".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWarning")]
        TiIcon::TiWarning => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.171 15.398l-5.912-9.854c-.776-1.293-1.963-2.033-3.259-2.033s-2.483.74-3.259 2.031l-5.912 9.856c-.786 1.309-.872 2.705-.235 3.83.636 1.126 1.878 1.772 3.406 1.772h12c1.528 0 2.77-.646 3.406-1.771.637-1.125.551-2.521-.235-3.831zm-9.171 2.151c-.854 0-1.55-.695-1.55-1.549 0-.855.695-1.551 1.55-1.551s1.55.696 1.55 1.551c0 .854-.696 1.549-1.55 1.549zm1.633-7.424c-.011.031-1.401 3.468-1.401 3.468-.038.094-.13.156-.231.156s-.193-.062-.231-.156l-1.391-3.438c-.09-.233-.129-.443-.129-.655 0-.965.785-1.75 1.75-1.75s1.75.785 1.75 1.75c0 .212-.039.422-.117.625z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWarning".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWarningOutline")]
        TiIcon::TiWarningOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 5.511c.561 0 1.119.354 1.544 1.062l5.912 9.854c.851 1.415.194 2.573-1.456 2.573h-12c-1.65 0-2.307-1.159-1.456-2.573l5.912-9.854c.425-.708.983-1.062 1.544-1.062m0-2c-1.296 0-2.482.74-3.259 2.031l-5.912 9.856c-.786 1.309-.872 2.705-.235 3.83s1.879 1.772 3.406 1.772h12c1.527 0 2.77-.646 3.406-1.771s.551-2.521-.235-3.83l-5.912-9.854c-.777-1.294-1.963-2.034-3.259-2.034z\" />\n<circle cx=\"12\" cy=\"16\" r=\"1.3\" />\n<path d=\"M13.5 10c0-.83-.671-1.5-1.5-1.5s-1.5.67-1.5 1.5c0 .199.041.389.111.562.554 1.376 1.389 3.438 1.389 3.438l1.391-3.438c.068-.173.109-.363.109-.562z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiWarningOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWatch")]
        TiIcon::TiWatch => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 13h2c.55 0 1-.45 1-1s-.45-1-1-1h-1v-1c0-.55-.45-1-1-1s-1 .45-1 1v2c0 .55.45 1 1 1zM17 7.105v-2.105c0-1.654-1.346-3-3-3h-4c-1.654 0-3 1.346-3 3v2.105c-1.236 1.263-2 2.989-2 4.895s.764 3.632 2 4.895v2.105c0 1.654 1.346 3 3 3h4c1.654 0 3-1.346 3-3v-2.105c1.236-1.262 2-2.988 2-4.895s-.764-3.632-2-4.895zm-8-2.105c0-.551.449-1 1-1h4c.551 0 1 .449 1 1v1.809c-.883-.512-1.906-.809-3-.809s-2.117.297-3 .809v-1.809zm6 14c0 .551-.449 1-1 1h-4c-.551 0-1-.449-1-1v-1.811c.883.513 1.906.811 3 .811s2.117-.298 3-.811v1.811zm-3-2c-2.757 0-5-2.243-5-5s2.243-5 5-5 5 2.243 5 5-2.243 5-5 5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWatch".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWaves")]
        TiIcon::TiWaves => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M15 19c-1.342 0-2.685-.511-3.707-1.532-1.266-1.265-3.323-1.264-4.586 0-.391.391-1.023.391-1.414 0s-.391-1.023 0-1.414c2.043-2.043 5.369-2.043 7.414 0 1.265 1.264 3.322 1.263 4.586 0 .391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-1.021 1.021-2.364 1.532-3.707 1.532zM15 15c-1.342 0-2.685-.511-3.707-1.532-1.266-1.265-3.323-1.264-4.586 0-.391.391-1.023.391-1.414 0s-.391-1.023 0-1.414c2.043-2.043 5.369-2.043 7.414 0 1.265 1.264 3.322 1.263 4.586 0 .391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-1.021 1.021-2.364 1.532-3.707 1.532zM15 11c-1.342 0-2.685-.511-3.707-1.532-1.266-1.265-3.323-1.264-4.586 0-.391.391-1.023.391-1.414 0s-.391-1.023 0-1.414c2.043-2.042 5.369-2.044 7.414 0 1.265 1.264 3.322 1.263 4.586 0 .391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-1.021 1.021-2.364 1.532-3.707 1.532z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWaves".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWavesOutline")]
        TiIcon::TiWavesOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.221 10.761c.498-.552.779-1.252.779-2 0-.801-.312-1.555-.879-2.121-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879c-.233.232-.546.361-.879.361-.333 0-.646-.129-.879-.362-1.366-1.366-3.185-2.118-5.121-2.118s-3.755.752-5.121 2.118c-.567.567-.879 1.321-.879 2.122 0 .748.281 1.448.779 2-.498.551-.779 1.252-.779 2s.281 1.448.779 2c-.498.551-.779 1.252-.779 2 0 .801.312 1.555.879 2.121.566.566 1.32.879 2.121.879s1.555-.312 2.121-.879c.234-.233.545-.362.878-.362.333 0 .646.129.88.363 1.367 1.365 3.185 2.117 5.121 2.117 1.937 0 3.755-.752 5.121-2.118.567-.567.879-1.32.879-2.121 0-.748-.281-1.448-.779-2 .498-.552.779-1.252.779-2s-.281-1.449-.779-2zm-1.514 6.707c-1.021 1.021-2.364 1.532-3.707 1.532-1.342 0-2.685-.511-3.707-1.532-.633-.632-1.463-.948-2.293-.948-.831 0-1.661.316-2.292.948-.196.195-.452.293-.708.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414 1.021-1.021 2.364-1.532 3.706-1.532 1.343 0 2.686.511 3.708 1.532.632.632 1.463.947 2.293.947.831 0 1.661-.315 2.293-.947.195-.195.451-.293.707-.293s.512.098.707.293c.391.39.391 1.023 0 1.414zm-13.414-9.414c1.021-1.022 2.365-1.533 3.707-1.533 1.343 0 2.685.511 3.707 1.532.632.633 1.463.948 2.293.948.831 0 1.661-.315 2.293-.947.195-.196.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414-1.021 1.021-2.364 1.532-3.707 1.532-1.342 0-2.685-.511-3.707-1.532-.633-.632-1.463-.948-2.293-.948-.831 0-1.661.316-2.292.948-.196.195-.452.293-.708.293s-.512-.098-.707-.293c-.391-.391-.391-1.024 0-1.414zm13.414 5.414c-1.021 1.021-2.364 1.532-3.707 1.532-1.342 0-2.685-.511-3.707-1.532-.633-.632-1.463-.948-2.293-.948-.831 0-1.661.316-2.292.948-.196.195-.452.293-.708.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414 1.021-1.021 2.364-1.532 3.706-1.532 1.343 0 2.686.511 3.708 1.532.632.632 1.463.947 2.293.947.831 0 1.661-.315 2.293-.947.195-.195.451-.293.707-.293s.512.098.707.293c.391.39.391 1.023 0 1.414z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWavesOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherCloudy")]
        TiIcon::TiWeatherCloudy => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17 19h-11c-2.206 0-4-1.794-4-4 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.638 5.65 4.015 2.942-.246 5.35 2.113 5.35 4.985 0 2.757-2.243 5-5 5zm-11.095-6.006c-1.008.006-1.905.903-1.905 2.006s.897 2 2 2h11c1.654 0 3-1.346 3-3s-1.346-3-3-3c-.243 0-.5.041-.81.13l-1.075.307-.186-1.103c-.325-1.932-1.977-3.334-3.929-3.334-2.206 0-4 1.794-4 4 0 .272.027.545.082.811l.244 1.199-1.421-.016z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherCloudy".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherDownpour")]
        TiIcon::TiWeatherDownpour => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M15 22c-.552 0-1-.447-1-1v-6c0-.553.448-1 1-1s1 .447 1 1v6c0 .553-.448 1-1 1zM9 22c-.552 0-1-.447-1-1v-6c0-.553.448-1 1-1s1 .447 1 1v6c0 .553-.448 1-1 1zM12 24c-.552 0-1-.447-1-1v-6c0-.553.448-1 1-1s1 .447 1 1v6c0 .553-.448 1-1 1zM6 18c-2.206 0-4-1.794-4-4 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.639 5.65 4.015 2.936-.244 5.35 2.113 5.35 4.985 0 2.241-1.507 4.223-3.666 4.819-.535.146-1.083-.166-1.23-.697-.147-.532.165-1.083.698-1.23 1.294-.358 2.198-1.547 2.198-2.892 0-1.654-1.346-3-3-3-.242 0-.499.041-.811.13l-1.074.306-.185-1.102c-.326-1.932-1.978-3.334-3.93-3.334-2.206 0-4 1.794-4 4 0 .272.027.545.082.808l.248 1.202-1.422-.016c-1.011.006-1.908.903-1.908 2.006s.897 2 2 2c.552 0 1 .447 1 1s-.448 1-1 1z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiWeatherDownpour".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherNight")]
        TiIcon::TiWeatherNight => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M10.5 20c-.861 0-1.71-.151-2.523-.451l-1.317-.485.89-1.087c1.275-1.56 1.95-3.454 1.95-5.477s-.675-3.917-1.951-5.477l-.89-1.087 1.317-.485c.814-.3 1.663-.451 2.524-.451 4.136 0 7.5 3.364 7.5 7.5s-3.364 7.5-7.5 7.5zm-.509-2.024c.169.016.339.024.509.024 3.032 0 5.5-2.468 5.5-5.5s-2.468-5.5-5.5-5.5c-.17 0-.34.008-.509.024.991 1.645 1.509 3.511 1.509 5.476s-.518 3.831-1.509 5.476z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherNight".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherPartlySunny")]
        TiIcon::TiWeatherPartlySunny => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M14.5 3l-1 3-1-3c-.184-.553.114-1.149.666-1.333.553-.185 1.15.114 1.334.666.075.226.07.458 0 .667zM19.864 6.05l-2.829 1.415 1.415-2.829c.261-.521.894-.731 1.414-.472.521.261.731.894.472 1.415-.107.212-.274.372-.472.471zM21.5 12l-3-1 3-1c.553-.185 1.149.114 1.334.667.184.552-.115 1.148-.668 1.333-.225.075-.457.069-.666 0zM8.55 4.636l1.415 2.829-2.829-1.415c-.521-.261-.732-.894-.472-1.414.261-.521.895-.731 1.414-.472.213.107.373.274.472.472zM17.776 12.342c.139-.424.224-.871.224-1.342 0-2.481-2.019-4.5-4.5-4.5-1.34 0-2.537.594-3.357 1.528l-.143-.028c-1.776 0-3.369.78-4.469 2.011-.24-.08-.472-.086-.697-.011-.553.185-.852.781-.668 1.333.057.167.158.299.277.411-.283.697-.443 1.458-.443 2.256l.002.126c-1.725.445-3.002 2.013-3.002 3.874 0 2.206 1.795 4 4 4h11c2.757 0 5-2.243 5-5 0-2.129-1.344-3.939-3.224-4.658zm-4.276-3.842c1.379 0 2.5 1.121 2.5 2.5 0 .366-.096.706-.238 1.019-.354.021-.72.074-1.118.188-.521-1.353-1.604-2.415-2.967-2.905.456-.49 1.102-.802 1.823-.802zm2.5 11.5h-11c-1.104 0-2-.897-2-2s.896-2 1.908-2.006l1.422.016-.248-1.202c-.055-.263-.082-.536-.082-.808 0-2.206 1.795-4 4-4l.069-.014c1.904.055 3.495 1.406 3.847 3.27l.038.186c.123.436.517.706.946.712l.289-.023c.312-.09.569-.131.811-.131 1.654 0 3 1.346 3 3s-1.346 3-3 3z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiWeatherPartlySunny".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherShower")]
        TiIcon::TiWeatherShower => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 18c-.552 0-1-.447-1-1s.448-1 1-1c1.654 0 3-1.346 3-3s-1.346-3-3-3c-.243 0-.5.041-.81.13l-1.075.307-.185-1.103c-.326-1.932-1.978-3.334-3.93-3.334-2.206 0-4 1.794-4 4 0 .272.027.545.082.811l.244 1.199-1.42-.016c-1.009.006-1.906.903-1.906 2.006s.897 2 2 2c.552 0 1 .447 1 1s-.448 1-1 1c-2.206 0-4-1.794-4-4 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.638 5.65 4.015 2.939-.244 5.35 2.113 5.35 4.985 0 2.757-2.243 5-5 5zM10.5 18l1-3 1 3c.184.553-.114 1.149-.667 1.333-.552.185-1.149-.114-1.333-.666-.075-.226-.07-.458 0-.667zM13.5 20l1-3 1 3c.184.553-.114 1.149-.667 1.333-.552.185-1.149-.114-1.333-.666-.075-.226-.07-.458 0-.667zM7.5 20l1-3 1 3c.184.553-.114 1.149-.667 1.333-.552.185-1.149-.114-1.333-.666-.075-.226-.07-.458 0-.667z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherShower".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherSnow")]
        TiIcon::TiWeatherSnow => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M20.5 15.134l-2.457-.503 1.483-.396c.533-.143.85-.69.707-1.225-.142-.533-.689-.85-1.225-.707l-1.508.403c.037-.231.071-.464.071-.706s-.034-.476-.071-.707l1.51.404.26.034c.441 0 .846-.295.965-.741.143-.533-.174-1.082-.707-1.225l-1.483-.397 2.455-.502c.216-.044.42-.156.577-.333.386-.436.347-1.102-.089-1.488-.436-.386-1.102-.347-1.488.089l-1.663 1.874.398-1.479c.144-.533-.173-1.082-.706-1.226-.531-.142-1.082.173-1.226.706l-.407 1.517c-.366-.299-.771-.544-1.219-.717l1.102-1.102c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-1.086 1.086.793-2.379c.069-.209.075-.441 0-.667-.184-.552-.781-.851-1.333-.666-.552.184-.85.78-.667 1.333l.793 2.379-1.086-1.086c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l1.102 1.102c-.447.173-.853.419-1.219.717l-.405-1.515c-.143-.534-.697-.852-1.224-.708-.534.143-.851.69-.708 1.224l.396 1.485-1.662-1.877c-.146-.164-.345-.285-.578-.333-.57-.117-1.127.25-1.244.82s.251 1.128.822 1.245l2.454.503-1.48.396c-.533.143-.85.691-.707 1.225.119.447.523.741.965.741l.26-.034 1.508-.404c-.039.231-.073.465-.073.706 0 .242.034.475.071.707l-1.508-.404c-.532-.142-1.081.173-1.225.707-.144.533.174 1.082.707 1.225l1.483.397-2.455.502c-.216.044-.42.156-.577.334-.387.436-.347 1.102.089 1.487.436.387 1.103.347 1.488-.089l1.665-1.878-.398 1.484c-.144.533.173 1.082.707 1.225l.26.034c.441 0 .845-.294.965-.741l.406-1.515c.366.298.771.544 1.22.716l-1.104 1.102c-.391.39-.391 1.023 0 1.414s1.023.391 1.414 0l.706-.707h.252l-.666 1.999c-.069.209-.075.441 0 .667.184.552.781.851 1.333.666.553-.184.851-.78.667-1.333l-.666-1.999h.252l.707.707c.196.195.451.293.707.293s.512-.098.707-.293c.391-.39.391-1.023 0-1.414l-1.102-1.103c.448-.172.854-.418 1.22-.717l.406 1.517c.12.447.523.741.965.741l.26-.034c.533-.143.851-.691.707-1.225l-.397-1.48 1.662 1.874c.146.165.345.285.577.333.57.117 1.128-.251 1.244-.821.117-.57-.251-1.127-.821-1.244zm-7.428-.634c-1.379 0-2.5-1.121-2.5-2.5s1.121-2.5 2.5-2.5 2.5 1.121 2.5 2.5-1.121 2.5-2.5 2.5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherSnow".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherStormy")]
        TiIcon::TiWeatherStormy => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M17 18c-.553 0-1-.447-1-1s.447-1 1-1c1.654 0 3-1.346 3-3s-1.346-3-3-3c-.238 0-.496.042-.813.131l-1.071.301-.186-1.098c-.326-1.932-1.979-3.334-3.93-3.334-2.205 0-4 1.794-4 4 0 .274.027.545.082.806l.26 1.24-1.436-.052c-1.01.006-1.906.903-1.906 2.006s.896 2 2 2c.553 0 1 .447 1 1s-.447 1-1 1c-2.205 0-4-1.794-4-4 0-1.861 1.277-3.429 3.002-3.874l-.002-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.638 5.649 4.015 2.925-.241 5.351 2.112 5.351 4.985 0 2.757-2.243 5-5 5zM12.639 14l-4.5 4.051 3 1.449-1.5 3.5 4.5-4.05-3-1.45z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherStormy".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherSunny")]
        TiIcon::TiWeatherSunny => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M13 4l-1 2.934-1-2.934c-.188-.553.106-1.152.659-1.341.552-.188 1.153.107 1.341.659.078.23.072.469 0 .682zM4 11l2.934 1-2.934 1c-.553.188-1.152-.106-1.341-.659-.188-.552.107-1.153.659-1.341.23-.078.469-.072.682 0zM11 20l1-2.934 1 2.934c.188.553-.106 1.152-.659 1.341-.552.188-1.152-.106-1.341-.659-.078-.23-.072-.469 0-.682zM20 12.998l-2.934-1 2.934-1c.553-.188 1.152.106 1.341.659.188.552-.106 1.152-.659 1.341-.23.078-.469.072-.682 0zM7.05 5.636l1.367 2.781-2.781-1.367c-.524-.257-.74-.891-.483-1.414.258-.523.891-.739 1.414-.482.218.107.383.28.483.482zM5.636 16.949l2.781-1.367-1.367 2.781c-.257.523-.891.739-1.414.482-.523-.258-.739-.891-.482-1.414.107-.218.28-.382.482-.482zM16.949 18.363l-1.367-2.781 2.781 1.367c.523.257.739.891.482 1.414-.258.523-.891.739-1.414.482-.218-.107-.382-.28-.482-.482zM18.362 7.048l-2.782 1.368 1.368-2.782c.257-.523.891-.739 1.414-.482.523.258.739.891.481 1.415-.106.217-.279.381-.481.481zM12 16.5c-2.481 0-4.5-2.019-4.5-4.5s2.019-4.5 4.5-4.5 4.5 2.019 4.5 4.5-2.019 4.5-4.5 4.5zm0-7c-1.379 0-2.5 1.121-2.5 2.5s1.121 2.5 2.5 2.5 2.5-1.121 2.5-2.5-1.121-2.5-2.5-2.5z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherSunny".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherWindy")]
        TiIcon::TiWeatherWindy => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M19 5c-.553 0-1 .447-1 1s.447 1 1 1c.552 0 1 .448 1 1s-.448 1-1 1h-11c-.553 0-1 .447-1 1s.447 1 1 1h6c.552 0 1 .448 1 1s-.448 1-1 1h-6.4c-1.654 0-3 1.346-3 3s1.346 3 3 3c.553 0 1-.447 1-1s-.447-1-1-1c-.552 0-1-.448-1-1s.448-1 1-1h6.4c1.654 0 3-1.346 3-3 0-.353-.072-.686-.185-1h2.185c1.654 0 3-1.346 3-3s-1.346-3-3-3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWeatherWindy".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWeatherWindyCloudy")]
        TiIcon::TiWeatherWindyCloudy => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<g>\n  <path d=\"M4.798 15.75c-.134 0-.27-.026-.4-.084-1.457-.639-2.398-2.077-2.398-3.666 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.932 0 5.413 2.104 5.902 5.001.092.544-.275 1.061-.82 1.152-.544.083-1.06-.276-1.152-.82-.326-1.931-1.979-3.333-3.93-3.333-2.206 0-4 1.794-4 4 0 .272.027.546.081.812l.259 1.27-1.431-.088c-1.012.006-1.909.903-1.909 2.006 0 .795.471 1.515 1.2 1.834.506.222.736.812.515 1.317-.164.375-.531.599-.917.599zM19 7c-.553 0-1 .447-1 1s.447 1 1 1c.552 0 1 .448 1 1s-.448 1-1 1h-9.6c-.553 0-1 .447-1 1s.447 1 1 1h4.6c.552 0 1 .448 1 1s-.448 1-1 1h-5c-1.654 0-3 1.346-3 3s1.346 3 3 3c.553 0 1-.447 1-1s-.447-1-1-1c-.552 0-1-.448-1-1s.448-1 1-1h5c1.654 0 3-1.346 3-3 0-.353-.072-.686-.185-1h2.185c1.654 0 3-1.346 3-3s-1.346-3-3-3z\" />\n</g>",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiWeatherWindyCloudy".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWiFi")]
        TiIcon::TiWiFi => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13.414 19.412c.783-.779.783-2.047 0-2.826-.781-.785-2.049-.785-2.828-.002-.783.783-.783 2.051 0 2.831.781.78 2.049.781 2.828-.003zM20.485 11.515c-.512 0-1.024-.195-1.414-.586-3.899-3.899-10.243-3.898-14.143 0-.782.781-2.048.78-2.829 0-.781-.781-.781-2.047 0-2.829 5.459-5.458 14.341-5.458 19.799 0 .781.781.781 2.047 0 2.828-.389.391-.901.587-1.413.587zM7.757 15.757c-.512 0-1.024-.195-1.414-.586-.781-.781-.781-2.047 0-2.828 3.118-3.119 8.194-3.119 11.313 0 .781.781.781 2.047 0 2.829-.781.781-2.047.781-2.829 0-1.559-1.56-4.097-1.559-5.657 0-.389.39-.901.585-1.413.585z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWiFi".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWiFiOutline")]
        TiIcon::TiWiFiOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M21.157 10.764c0-.785-.269-1.464-.706-2.048-.045-.094-.131-.149-.21-.226-.163-.18-.341-.338-.536-.48-4.45-3.739-10.965-3.735-15.414.006-.193.142-.742.738-.742.738-.437.584-.706 1.305-.706 2.09 0 .816.362 1.758.759 2.155l5.775 5.796c.642.732 1.572 1.204 2.622 1.204.996 0 1.709-.167 2.526-1 .004 0 5.565-5.646 5.565-5.646.706-.703 1.067-1.699 1.067-2.589zm-9.156 7.234c-.829.002-1.501-.668-1.501-1.498-.002-.828.67-1.502 1.501-1.502.829-.002 1.501.67 1.499 1.502.002.828-.67 1.5-1.499 1.498zm3.888-3.268c-.293.293-.677.438-1.061.438-.385 0-.768-.146-1.061-.438-.976-.976-2.562-.976-3.536 0-.586.586-1.536.584-2.122 0-.586-.586-.586-1.537 0-2.123 2.144-2.144 5.632-2.144 7.779 0 .587.586.587 1.538.001 2.123zm2.829-2.828c-.293.293-.677.438-1.061.438s-.769-.146-1.062-.438c-2.533-2.534-6.658-2.534-9.192 0-.586.584-1.536.584-2.122 0-.585-.586-.585-1.536 0-2.123 3.704-3.701 9.729-3.701 13.435 0 .587.588.587 1.537.002 2.123z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWiFiOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWine")]
        TiIcon::TiWine => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M17.568 9.432c0-2.55-.906-5.592-.944-5.72-.128-.423-.517-.712-.958-.712h-7.332c-.441 0-.83.289-.958.712-.038.128-.944 3.17-.944 5.72 0 2.735 1.984 5.011 4.587 5.477l-.019.091v4h-1c-.553 0-1 .447-1 1s.447 1 1 1h4c.553 0 1-.447 1-1s-.447-1-1-1h-1v-4l-.019-.092c2.603-.466 4.587-2.741 4.587-5.476zm-5.568 3.568c-1.773 0-3.236-1.303-3.511-3h7.021c-.274 1.697-1.737 3-3.51 3zm-3.555-4c.062-1.468.422-3.093.653-4h5.803c.231.907.591 2.532.653 4h-7.109z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWine".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWorld")]
        TiIcon::TiWorld => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 2c-4.971 0-9 4.029-9 9s4.029 9 9 9 9-4.029 9-9-4.029-9-9-9zm2 2c0 1-.5 2-1.5 2s-1.5 1-1.5 2v3s1 0 1-3c0-.553.447-1 1-1s1 .447 1 1v3c-.552 0-1 .448-1 1s.448 1 1 1c.553 0 1-.448 1-1h1v-2l1 1-1 1c0 3 0 3-2 4 0-1-1-1-3-1v-2l-2-2v-2c-1 0-1 1-1 1l-.561-.561-2.39-2.39c.11-.192.225-.382.35-.564l.523-.678c1.468-1.716 3.644-2.807 6.078-2.807.691 0 1.359.098 2 .262v.738z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWorld".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiWorldOutline")]
        TiIcon::TiWorldOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M12 2c-4.971 0-9 4.029-9 9s4.029 9 9 9 9-4.029 9-9-4.029-9-9-9zm0 6c0-.553.447-1 1-1s1 .447 1 1v3c-.552 0-1 .448-1 1s.448 1 1 1c.553 0 1-.448 1-1h1v-2l1 1-1 1c0 3 0 3-2 4 0-1-1-1-3-1v-2l-2-2v-2c-1 0-1 1-1 1l-.561-.561-1.652-1.651c1.167-2.247 3.512-3.788 6.213-3.788.688 0 1.353.104 1.981.29-.086.895-.579 1.71-1.481 1.71-1 0-1.5 1-1.5 2v3s1 0 1-3zm0 10c-3.859 0-7-3.14-7-7 0-.776.133-1.521.367-2.219l1.926 1.926 1 1 1.707 1.707v1.586c0 .552.447 1 1 1 .779 0 1.651 0 2.006.091.038.301.209.582.468.742.168.104.36.16.552.16.145 0 .289-.032.422-.098 2.348-1.174 2.539-1.644 2.552-4.479l.708-.708c.391-.391.391-1.023 0-1.414l-1-1c-.192-.192-.448-.294-.708-.294-.129 0-.259.025-.383.076-.373.155-.617.52-.617.924v-2c0-.689-.351-1.298-.883-1.658.421-.411.712-.995.826-1.685 2.392 1.115 4.057 3.535 4.057 6.343 0 3.86-3.141 7-7 7z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiWorldOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiZoom")]
        TiIcon::TiZoom => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 4c-3.859 0-7 3.141-7 7 0 .763.127 1.495.354 2.183l-.749.75-.511.512-1.008 1.045c-.562.557-.891 1.345-.891 2.185 0 1.727 1.404 3.131 3.13 3.131.757 0 1.504-.278 2.104-.784l.064-.055.061-.061 1.512-1.51.75-.749c.688.226 1.421.353 2.184.353 3.859 0 7-3.141 7-7s-3.141-7-7-7zm0 12c-2.757 0-5-2.243-5-5s2.243-5 5-5 5 2.243 5 5-2.243 5-5 5zM13 7c-2.205 0-4 1.794-4 4s1.795 4 4 4 4-1.794 4-4-1.795-4-4-4zm0 7c-1.656 0-3-1.344-3-3s1.344-3 3-3 3 1.344 3 3-1.344 3-3 3z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiZoom".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiZoomIn")]
        TiIcon::TiZoomIn => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 11h-2v-2c0-.276-.224-.5-.5-.5s-.5.224-.5.5v2h-2c-.276 0-.5.224-.5.5s.224.5.5.5h2v2c0 .276.224.5.5.5s.5-.224.5-.5v-2h2c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM18.432 14.97l-.536-.537-.749-.75c.227-.688.354-1.42.354-2.183 0-3.859-3.141-7-7-7s-7 3.141-7 7 3.141 7 7 7c.763 0 1.496-.127 2.184-.354l.75.749 1.512 1.51.061.061.064.055c.601.506 1.348.784 2.104.784 1.726 0 3.13-1.404 3.13-3.131 0-.84-.328-1.628-.924-2.218l-.95-.986zm-12.932-3.47c0-2.757 2.243-5 5-5s5 2.243 5 5-2.243 5-5 5-5-2.243-5-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiZoomIn".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiZoomInOutline")]
        TiIcon::TiZoomInOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 11h-2v-2c0-.275-.225-.5-.5-.5s-.5.225-.5.5v2h-2c-.275 0-.5.225-.5.5s.225.5.5.5h2v2c0 .275.225.5.5.5s.5-.225.5-.5v-2h2c.275 0 .5-.225.5-.5s-.225-.5-.5-.5zM19.381 15.956l-2.244-2.283c.227-.687.363-1.412.363-2.173 0-3.859-3.141-7-7-7s-7 3.141-7 7 3.141 7 7 7c.762 0 1.488-.137 2.173-.364l2.397 2.386c.601.506 1.348.783 2.104.783 1.727 0 3.131-1.404 3.131-3.131 0-.84-.328-1.628-.924-2.218zm-3.901-1.11l2.492 2.531c.205.203.332.486.332.797 0 .625-.507 1.131-1.131 1.131-.312 0-.594-.127-.816-.313l-2.512-2.511c.646-.436 1.201-.991 1.635-1.635zm-9.98-3.346c0-2.757 2.243-5 5-5s5 2.243 5 5-2.243 5-5 5-5-2.243-5-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiZoomInOutline".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiZoomOut")]
        TiIcon::TiZoomOut => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 11h-5c-.276 0-.5.224-.5.5s.224.5.5.5h5c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM19.381 15.956l-.949-.986-.537-.537-.749-.75c.227-.688.354-1.42.354-2.183 0-3.859-3.14-7-7-7s-7 3.141-7 7 3.14 7 7 7c.763 0 1.496-.127 2.184-.354l.75.749 1.512 1.51.06.061.065.055c.601.506 1.348.784 2.104.784 1.726 0 3.13-1.404 3.13-3.131 0-.84-.328-1.628-.924-2.218zm-13.881-4.456c0-2.757 2.243-5 5-5s5 2.243 5 5-2.243 5-5 5-5-2.243-5-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiZoomOut".to_owned())),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiZoomOutOutline")]
        TiIcon::TiZoomOutOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M13 12h-5c-.275 0-.5-.225-.5-.5s.225-.5.5-.5h5c.275 0 .5.225.5.5s-.225.5-.5.5zM19.381 15.956l-2.245-2.283c.228-.687.364-1.412.364-2.173 0-3.859-3.141-7-7-7s-7 3.141-7 7 3.141 7 7 7c.761 0 1.488-.137 2.173-.364l2.397 2.386c.601.506 1.348.783 2.104.783 1.727 0 3.131-1.404 3.131-3.131 0-.84-.328-1.628-.924-2.218zm-3.901-1.11l2.492 2.531c.205.203.332.486.332.797 0 .625-.507 1.131-1.131 1.131-.312 0-.594-.127-.816-.313l-2.512-2.511c.646-.436 1.201-.991 1.635-1.635zm-9.98-3.346c0-2.757 2.243-5 5-5s5 2.243 5 5-2.243 5-5 5-5-2.243-5-5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(
                                title.unwrap_or_else(|| "TiZoomOutOutline".to_owned()),
                            ),
                    ),
                cx,
            )
        }
        #[cfg(feature = "TiZoomOutline")]
        TiIcon::TiZoomOutline => {
            leptos::IntoView::into_view(
                leptos::svg::svg(cx)
                    .classes(class)
                    .attr("style", format!(" {}", style))
                    .attr("width", width)
                    .attr("height", height)
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "currentColor")
                    .attr("role", "graphics-symbol")
                    .inner_html(
                        "<path d=\"M14 8c1.656 0 3 1.344 3 3s-1.344 3-3 3-3-1.344-3-3 1.344-3 3-3m0-1c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4zM4.195 17.674c0 1.727 1.404 3.131 3.131 3.131.756 0 1.503-.277 2.104-.783l2.397-2.386c.685.227 1.412.364 2.173.364 3.86 0 7-3.141 7-7s-3.14-7-7-7c-3.859 0-7 3.141-7 7 0 .761.136 1.486.364 2.173l-2.245 2.283c-.596.59-.924 1.378-.924 2.218zm6.459-1.694l-2.512 2.511c-.223.187-.504.313-.816.313-.624 0-1.131-.506-1.131-1.131 0-.311.127-.594.332-.797l2.492-2.531c.435.645.99 1.2 1.635 1.635zm3.346.02c-2.757 0-5-2.243-5-5s2.243-5 5-5 5 2.243 5 5-2.243 5-5 5z\" />",
                    )
                    .child(
                        leptos::svg::title(cx)
                            .child(title.unwrap_or_else(|| "TiZoomOutline".to_owned())),
                    ),
                cx,
            )
        }
    }
}
