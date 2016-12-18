//
//  UIChannel.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIChannel.hpp"

#include <iostream>

UIChannel::UIChannel(Stage* stage, ChannelAddress channelAddress): UIControlElement(stage, {}) {
    setChannelAddress(channelAddress);
    m_slider = std::make_shared<Slider>(Slider(0, 255, [this](double x) -> void { this->activate(); }, [this]() -> void { this->deactivate(); }, m_stage->getFont()));
    m_uiParts.push_back(m_slider);
}

void UIChannel::setChannelAddress(ChannelAddress channelAddress) {
    m_channelAddress = channelAddress;
}

void UIChannel::action() {
    m_stage->startFade(m_channelAddress, m_fadeTime, m_slider->getValue(), m_fadeCurve, m_id);
}
