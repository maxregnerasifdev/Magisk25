#!/bin/bash

# Samsung Galaxy S22+ (SM-S906B) Testing Script for Magisk25
# Tests device detection and patching functionality

set -e

echo "=== Samsung Galaxy S22+ Magisk Testing Script ==="
echo "Testing device: SM-S906B (Galaxy S22+)"
echo "Target: OneUI 8 and Exynos 2200 support"
echo

# Test device detection
echo "1. Testing device detection..."
if command -v getprop >/dev/null 2>&1; then
    DEVICE=$(getprop ro.product.device 2>/dev/null || echo "unknown")
    MODEL=$(getprop ro.product.model 2>/dev/null || echo "unknown")
    BRAND=$(getprop ro.product.brand 2>/dev/null || echo "unknown")
    ONEUI_VER=$(getprop ro.build.version.oneui 2>/dev/null || echo "unknown")
    KNOX_STATUS=$(getprop ro.boot.warranty_bit 2>/dev/null || echo "unknown")
    
    echo "   Device: $DEVICE"
    echo "   Model: $MODEL"
    echo "   Brand: $BRAND"
    echo "   OneUI Version: $ONEUI_VER"
    echo "   Knox Status: $KNOX_STATUS"
    
    # Check if this is a Samsung S22+
    if [[ "$DEVICE" =~ (s906|dm3q|b0q) ]] && [[ "$MODEL" =~ SM-S906 ]] && [[ "$BRAND" =~ [Ss]amsung ]]; then
        echo "   ✅ Samsung Galaxy S22+ detected!"
    else
        echo "   ⚠️  Not a Samsung Galaxy S22+ device"
    fi
else
    echo "   ⚠️  getprop not available (not running on Android)"
fi

echo

# Test boot image validation
echo "2. Testing boot image validation..."
if [ -f "/dev/block/bootdevice/by-name/boot" ]; then
    echo "   Found boot partition: /dev/block/bootdevice/by-name/boot"
    # Check for Samsung signatures in boot image
    if dd if=/dev/block/bootdevice/by-name/boot bs=1024 count=1 2>/dev/null | grep -q "SAMSUNG\|EXYNOS\|OneUI"; then
        echo "   ✅ Samsung boot image signatures found"
    else
        echo "   ⚠️  Samsung signatures not found in boot image"
    fi
else
    echo "   ⚠️  Boot partition not accessible"
fi

echo

# Test Magisk binary functionality
echo "3. Testing Magisk binary..."
if [ -f "./magiskboot" ]; then
    echo "   Found magiskboot binary"
    if ./magiskboot 2>&1 | grep -q "usage"; then
        echo "   ✅ magiskboot is functional"
    else
        echo "   ❌ magiskboot is not working"
    fi
else
    echo "   ⚠️  magiskboot binary not found"
fi

echo

# Test Samsung-specific patches
echo "4. Testing Samsung patch functionality..."
if [ -f "test_boot.img" ]; then
    echo "   Testing with test_boot.img..."
    if ./magiskboot unpack test_boot.img 2>/dev/null; then
        echo "   ✅ Boot image unpacked successfully"
        
        # Test Samsung patching
        if [ -f "ramdisk.cpio" ]; then
            echo "   Testing Samsung ramdisk patching..."
            if ./magiskboot cpio ramdisk.cpio patch 2>/dev/null; then
                echo "   ✅ Samsung ramdisk patching completed"
            else
                echo "   ❌ Samsung ramdisk patching failed"
            fi
        fi
        
        # Clean up
        rm -f kernel dtb ramdisk.cpio recovery_dtbo extra
    else
        echo "   ❌ Failed to unpack boot image"
    fi
else
    echo "   ⚠️  No test boot image available"
fi

echo

# Test Knox bypass
echo "5. Testing Knox bypass functionality..."
if command -v getprop >/dev/null 2>&1; then
    KNOX_API=$(getprop ro.config.knox 2>/dev/null || echo "")
    WARRANTY_BIT=$(getprop ro.boot.warranty_bit 2>/dev/null || echo "")
    
    if [ -n "$KNOX_API" ]; then
        echo "   Knox API detected: $KNOX_API"
        if [ "$WARRANTY_BIT" = "0" ]; then
            echo "   ✅ Knox warranty bit intact (0)"
        else
            echo "   ⚠️  Knox warranty bit modified ($WARRANTY_BIT)"
        fi
    else
        echo "   ⚠️  Knox not detected or not available"
    fi
else
    echo "   ⚠️  Cannot test Knox (not on Android)"
fi

echo

# Test OneUI 8 compatibility
echo "6. Testing OneUI 8 compatibility..."
if command -v getprop >/dev/null 2>&1; then
    ANDROID_VER=$(getprop ro.build.version.release 2>/dev/null || echo "unknown")
    API_LEVEL=$(getprop ro.build.version.sdk 2>/dev/null || echo "unknown")
    ONEUI_VER=$(getprop ro.build.version.oneui 2>/dev/null || echo "unknown")
    
    echo "   Android Version: $ANDROID_VER"
    echo "   API Level: $API_LEVEL"
    echo "   OneUI Version: $ONEUI_VER"
    
    # Check for OneUI 8 (typically Android 14+)
    if [ "$API_LEVEL" -ge 34 ] 2>/dev/null; then
        echo "   ✅ Compatible with OneUI 8 (Android 14+)"
    else
        echo "   ⚠️  May not be OneUI 8 compatible"
    fi
else
    echo "   ⚠️  Cannot test OneUI version (not on Android)"
fi

echo

# Test Exynos 2200 detection
echo "7. Testing Exynos 2200 detection..."
if command -v getprop >/dev/null 2>&1; then
    SOC_MODEL=$(getprop ro.soc.model 2>/dev/null || echo "unknown")
    CHIPSET=$(getprop ro.chipname 2>/dev/null || echo "unknown")
    
    echo "   SoC Model: $SOC_MODEL"
    echo "   Chipset: $CHIPSET"
    
    if [[ "$SOC_MODEL" =~ 2200 ]] || [[ "$CHIPSET" =~ 2200 ]]; then
        echo "   ✅ Exynos 2200 detected"
    else
        echo "   ⚠️  Exynos 2200 not detected"
    fi
else
    echo "   ⚠️  Cannot test chipset (not on Android)"
fi

echo

echo "=== Samsung Galaxy S22+ Testing Complete ==="
echo "Review the results above to ensure compatibility."
echo "For issues, check the Magisk logs and Samsung-specific patches."

